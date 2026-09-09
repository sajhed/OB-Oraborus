use super::{capabilities_from_name,AiProvider,ChatOutput,ChatRequest,ModelInfo,TokenSink,VisionInput};
use crate::{db::Database,error::{ObError,ObResult},models::{CapabilityState,ProviderStatus}};
use async_trait::async_trait;
use base64::Engine;
use futures_util::StreamExt;
use serde_json::{json,Value};
use tokio_util::sync::CancellationToken;

pub struct OllamaProvider{db:Database,client:reqwest::Client}
impl OllamaProvider{
    pub fn new(db:Database)->Self{Self{db,client:reqwest::Client::builder().connect_timeout(std::time::Duration::from_secs(2)).timeout(std::time::Duration::from_secs(180)).build().expect("HTTP client")}}
    fn base(&self)->String{self.db.execute(|connection|Ok(connection.query_row("SELECT COALESCE(base_url,'http://127.0.0.1:11434') FROM providers WHERE id='ollama'",[],|row|row.get(0))?)).unwrap_or_else(|_|"http://127.0.0.1:11434".into()).trim_end_matches('/').to_string()}
    fn output(value:Value,model:&str)->ChatOutput{ChatOutput{text:value.pointer("/message/content").and_then(Value::as_str).unwrap_or_default().into(),model:value.get("model").and_then(Value::as_str).unwrap_or(model).into(),usage:Some(json!({"promptTokens":value.get("prompt_eval_count"),"completionTokens":value.get("eval_count")})),tool_calls:value.pointer("/message/tool_calls").and_then(Value::as_array).cloned().unwrap_or_default(),finish_reason:value.get("done_reason").and_then(Value::as_str).map(str::to_string)}}
}
#[async_trait]
impl AiProvider for OllamaProvider{
    fn id(&self)->&'static str{"ollama"} fn label(&self)->&'static str{"Ollama"}
    fn capabilities(&self)->Vec<String>{vec!["chat".into(),"streaming".into(),"vision_model_dependent".into(),"embeddings_model_dependent".into(),"tool_calling_model_dependent".into()]}
    async fn test_connection(&self)->ObResult<ProviderStatus>{
        let response=self.client.get(format!("{}/api/tags",self.base())).send().await.map_err(|_|ObError::NotConnected("Ollama is not running at the configured endpoint".into()))?;
        if !response.status().is_success(){return Err(ObError::NotConnected(format!("Ollama returned HTTP {}",response.status())))}
        let models=self.list_models().await?; let active=self.db.execute(|connection|Ok(connection.query_row("SELECT active_model FROM providers WHERE id='ollama'",[],|row|row.get::<_,Option<String>>(0))?)).unwrap_or(None).or_else(||models.first().map(|m|m.id.clone()));
        Ok(ProviderStatus{id:self.id().into(),label:self.label().into(),state:CapabilityState::Available,active_model:active,capabilities:self.capabilities(),latency_ms:None,detail:Some(format!("{} model(s) installed",models.len()))})
    }
    async fn list_models(&self)->ObResult<Vec<ModelInfo>>{
        let value:Value=self.client.get(format!("{}/api/tags",self.base())).send().await.map_err(|_|ObError::NotConnected("Ollama is not running".into()))?.error_for_status()?.json().await?;
        Ok(value.get("models").and_then(Value::as_array).into_iter().flatten().filter_map(|item|{let name=item.get("name")?.as_str()?.to_string();Some(ModelInfo{id:name.clone(),name:name.clone(),capabilities:capabilities_from_name(&name),context_length:None,size_bytes:item.get("size").and_then(Value::as_u64),local:true})}).collect())
    }
    async fn chat(&self,request:ChatRequest,cancellation:CancellationToken)->ObResult<ChatOutput>{
        let mut messages:Vec<Value>=request.messages.iter().map(|m|json!({"role":m.role,"content":m.content})).collect(); if let Some(system)=&request.system{messages.insert(0,json!({"role":"system","content":system}))}
        let body=json!({"model":request.model,"messages":messages,"stream":false,"tools":request.tools,"options":{"temperature":request.temperature,"num_predict":request.max_tokens}});
        let future=self.client.post(format!("{}/api/chat",self.base())).json(&body).send(); let response=tokio::select!{_ = cancellation.cancelled()=>return Err(ObError::Cancelled),value=future=>value?};
        let value:Value=response.error_for_status()?.json().await?; Ok(Self::output(value,&request.model))
    }
    async fn stream_chat(&self,request:ChatRequest,sink:TokenSink,cancellation:CancellationToken)->ObResult<ChatOutput>{
        let model=request.model.clone(); let mut messages:Vec<Value>=request.messages.iter().map(|m|json!({"role":m.role,"content":m.content})).collect(); if let Some(system)=&request.system{messages.insert(0,json!({"role":"system","content":system}))}
        let response=self.client.post(format!("{}/api/chat",self.base())).json(&json!({"model":model,"messages":messages,"stream":true,"tools":request.tools,"options":{"temperature":request.temperature,"num_predict":request.max_tokens}})).send().await?.error_for_status()?;
        let mut stream=response.bytes_stream(); let mut buffer=String::new(); let mut text=String::new(); let mut final_value=json!({});
        loop{tokio::select!{_ = cancellation.cancelled()=>return Err(ObError::Cancelled),chunk=stream.next()=>match chunk{Some(Ok(bytes))=>{buffer.push_str(&String::from_utf8_lossy(&bytes));while let Some(index)=buffer.find('\n'){let line=buffer[..index].trim().to_string();buffer.drain(..=index);if line.is_empty(){continue}if let Ok(value)=serde_json::from_str::<Value>(&line){if let Some(token)=value.pointer("/message/content").and_then(Value::as_str){text.push_str(token);sink(token.into())}if value.get("done").and_then(Value::as_bool)==Some(true){final_value=value}}}},Some(Err(error))=>return Err(error.into()),None=>break}}}
        let mut output=Self::output(final_value,&model);output.text=text;Ok(output)
    }
    async fn vision(&self,input:VisionInput,cancellation:CancellationToken)->ObResult<ChatOutput>{
        let request=ChatRequest{model:input.model.clone(),messages:vec![super::ChatMessage{role:"user".into(),content:input.prompt}],temperature:.2,max_tokens:2048,system:None,tools:vec![]};
        let body=json!({"model":request.model,"messages":[{"role":"user","content":request.messages[0].content,"images":[input.data_base64]}],"stream":false});
        let response=tokio::select!{_ = cancellation.cancelled()=>return Err(ObError::Cancelled),value=self.client.post(format!("{}/api/chat",self.base())).json(&body).send()=>value?}; let value:Value=response.error_for_status()?.json().await?;Ok(Self::output(value,&input.model))
    }
    async fn embeddings(&self,model:&str,input:&[String],cancellation:CancellationToken)->ObResult<Vec<Vec<f32>>>{
        let response=tokio::select!{_ = cancellation.cancelled()=>return Err(ObError::Cancelled),value=self.client.post(format!("{}/api/embed",self.base())).json(&json!({"model":model,"input":input})).send()=>value?};let value:Value=response.error_for_status()?.json().await?;
        serde_json::from_value(value.get("embeddings").cloned().ok_or_else(||ObError::Unavailable("Selected Ollama model did not return embeddings".into()))?).map_err(Into::into)
    }
    async fn tool_calling(&self,request:ChatRequest,cancellation:CancellationToken)->ObResult<ChatOutput>{self.chat(request,cancellation).await}
    async fn cancel(&self)->ObResult<()>{Ok(())}
}
