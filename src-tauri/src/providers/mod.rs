mod gemini;
mod ollama;
mod openai_compatible;

use crate::{db::Database, error::{ObError, ObResult}, models::{CapabilityState, ProviderStatus}, secrets::SecretVault};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, sync::Arc, time::Instant};
use tokio_util::sync::CancellationToken;

pub use gemini::GeminiProvider;
pub use ollama::OllamaProvider;
pub use openai_compatible::OpenAiCompatibleProvider;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(default="default_temperature")] pub temperature: f32,
    #[serde(default="default_tokens")] pub max_tokens: u32,
    #[serde(default)] pub system: Option<String>,
    #[serde(default)] pub tools: Vec<Value>,
}
fn default_temperature()->f32{0.4} fn default_tokens()->u32{2048}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage { pub role:String, pub content:String }

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ChatOutput { pub text:String, pub model:String, pub usage:Option<Value>, pub tool_calls:Vec<Value>, pub finish_reason:Option<String> }

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ModelInfo { pub id:String, pub name:String, pub capabilities:Vec<String>, pub context_length:Option<u64>, pub size_bytes:Option<u64>, pub local:bool }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionInput { pub model:String, pub prompt:String, pub mime_type:String, pub data_base64:String }

pub type TokenSink = Arc<dyn Fn(String) + Send + Sync>;

#[async_trait]
pub trait AiProvider: Send + Sync {
    fn id(&self) -> &'static str;
    fn label(&self) -> &'static str;
    fn capabilities(&self) -> Vec<String>;
    async fn connect(&self) -> ObResult<ProviderStatus> { self.test_connection().await }
    async fn test_connection(&self) -> ObResult<ProviderStatus>;
    async fn list_models(&self) -> ObResult<Vec<ModelInfo>>;
    async fn chat(&self, request: ChatRequest, cancellation: CancellationToken) -> ObResult<ChatOutput>;
    async fn stream_chat(&self, request: ChatRequest, sink: TokenSink, cancellation: CancellationToken) -> ObResult<ChatOutput>;
    async fn vision(&self, input: VisionInput, cancellation: CancellationToken) -> ObResult<ChatOutput>;
    async fn embeddings(&self, model: &str, input: &[String], cancellation: CancellationToken) -> ObResult<Vec<Vec<f32>>>;
    async fn tool_calling(&self, request: ChatRequest, cancellation: CancellationToken) -> ObResult<ChatOutput>;
    async fn cancel(&self) -> ObResult<()> { Ok(()) }
    async fn health_check(&self) -> ObResult<ProviderStatus> { self.test_connection().await }
}

#[derive(Debug, Clone)]
pub struct ProviderConfig { pub id:String, pub base_url:String, pub enabled:bool, pub active_model:Option<String> }

#[derive(Clone)]
pub struct ProviderManager { db:Database, providers:HashMap<String,Arc<dyn AiProvider>> }
impl ProviderManager {
    pub fn new(db:Database, vault:SecretVault) -> Self {
        let mut providers:HashMap<String,Arc<dyn AiProvider>> = HashMap::new();
        providers.insert("ollama".into(),Arc::new(OllamaProvider::new(db.clone())));
        providers.insert("gemini".into(),Arc::new(GeminiProvider::new(db.clone(),vault.clone())));
        providers.insert("openai-compatible".into(),Arc::new(OpenAiCompatibleProvider::new(db.clone(),vault)));
        Self{db,providers}
    }
    pub fn config(&self,id:&str)->ObResult<ProviderConfig>{
        self.db.execute(|connection|{
            let result=connection.query_row("SELECT id,COALESCE(base_url,''),enabled,active_model FROM providers WHERE id=?1",[id],|row|Ok(ProviderConfig{id:row.get(0)?,base_url:row.get(1)?,enabled:row.get::<_,i64>(2)?!=0,active_model:row.get(3)?}));
            result.map_err(Into::into)
        })
    }
    pub fn provider(&self,id:&str)->ObResult<Arc<dyn AiProvider>>{ self.providers.get(id).cloned().ok_or_else(||ObError::Validation(format!("Unknown provider: {id}"))) }
    pub async fn health_all(&self)->Vec<ProviderStatus>{
        let mut output=Vec::new();
        for (id,provider) in &self.providers {
            let config=self.config(id);
            if let Ok(config)=config {
                if !config.enabled { output.push(ProviderStatus{id:id.clone(),label:provider.label().into(),state:CapabilityState::Disabled,active_model:config.active_model,capabilities:provider.capabilities(),latency_ms:None,detail:Some("Provider is disabled".into())}); continue; }
            }
            let started=Instant::now();
            match tokio::time::timeout(std::time::Duration::from_secs(3),provider.health_check()).await {
                Ok(Ok(mut status))=>{status.latency_ms=Some(started.elapsed().as_millis() as u64);output.push(status)},
                Ok(Err(error))=>output.push(ProviderStatus{id:id.clone(),label:provider.label().into(),state:match &error{ObError::NotConnected(_)=>CapabilityState::NotConnected,ObError::RequiresConfiguration(_)=>CapabilityState::RequiresConfiguration,_=>CapabilityState::Unavailable},active_model:self.config(id).ok().and_then(|c|c.active_model),capabilities:provider.capabilities(),latency_ms:Some(started.elapsed().as_millis() as u64),detail:Some(error.to_string())}),
                Err(_)=>output.push(ProviderStatus{id:id.clone(),label:provider.label().into(),state:CapabilityState::Unavailable,active_model:self.config(id).ok().and_then(|c|c.active_model),capabilities:provider.capabilities(),latency_ms:None,detail:Some("Health check timed out".into())})
            }
        }
        output.sort_by(|a,b|a.id.cmp(&b.id)); output
    }
    pub async fn list_models(&self,id:&str)->ObResult<Vec<ModelInfo>>{ self.provider(id)?.list_models().await }
    pub async fn chat(&self,id:&str,request:ChatRequest,cancellation:CancellationToken)->ObResult<ChatOutput>{ self.provider(id)?.chat(request,cancellation).await }
    pub async fn stream_chat(&self,id:&str,request:ChatRequest,sink:TokenSink,cancellation:CancellationToken)->ObResult<ChatOutput>{ self.provider(id)?.stream_chat(request,sink,cancellation).await }
    pub async fn embed(&self,id:&str,model:&str,input:&[String],cancellation:CancellationToken)->ObResult<Vec<Vec<f32>>>{self.provider(id)?.embeddings(model,input,cancellation).await}
}

pub fn capabilities_from_name(name:&str)->Vec<String>{
    let lower=name.to_lowercase(); let mut output=vec!["chat".into(),"streaming".into()];
    if ["vision","llava","moondream","qwen2.5-vl","gemini","gpt-4o"].iter().any(|token|lower.contains(token)){output.push("vision".into())}
    if ["embed","nomic","bge","minilm","text-embedding"].iter().any(|token|lower.contains(token)){output.push("embeddings".into())}
    if ["tool","function","qwen","llama3","mistral","gemini","gpt"].iter().any(|token|lower.contains(token)){output.push("tool_calling".into())}
    if ["code","coder","deepseek","starcoder"].iter().any(|token|lower.contains(token)){output.push("coding".into())}
    output.sort(); output.dedup(); output
}
