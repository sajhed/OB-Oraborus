use crate::{db::Database,error::{ObError,ObResult},providers::ProviderManager};
use serde::{Deserialize,Serialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct RouteDecision{pub provider_id:String,pub model:String,pub reason:String,pub required_capabilities:Vec<String>,pub fallback_chain:Vec<String>}

#[derive(Clone)] pub struct ModelRouter{db:Database,providers:ProviderManager}
impl ModelRouter{
 pub fn new(db:Database,providers:ProviderManager)->Self{Self{db,providers}}
 pub async fn route(&self,text:&str,attachments:&[String])->ObResult<RouteDecision>{
  let lower=text.to_lowercase();let local_only:bool=self.db.get_setting("local_only",false);let private=local_only||["private","confidential","secret","local only"].iter().any(|v|lower.contains(v));
  let mut required=vec!["chat".to_string()];if !attachments.is_empty()||["screen","image","photo","screenshot"].iter().any(|v|lower.contains(v)){required.push("vision".into())}if ["code","rust","typescript","debug","build","repository"].iter().any(|v|lower.contains(v)){required.push("coding".into())}if ["tool","open ","create ","organize","run "].iter().any(|v|lower.contains(v)){required.push("tool_calling".into())}
  let order=if private{vec!["ollama","openai-compatible"]}else if required.contains(&"vision".into()){vec!["gemini","ollama","openai-compatible"]}else if required.contains(&"coding".into()){vec!["ollama","openai-compatible","gemini"]}else{vec!["ollama","gemini","openai-compatible"]};
  let mut fallback=Vec::new();
  for id in order{let config=match self.providers.config(id){Ok(v) if v.enabled=>v,_=>continue};let models=match self.providers.list_models(id).await{Ok(v)=>v,Err(_)=>continue};let selected=config.active_model.and_then(|active|models.iter().find(|m|m.id==active).cloned()).or_else(||models.iter().find(|m|required.iter().all(|cap|cap=="chat"||m.capabilities.contains(cap))).cloned()).or_else(||models.first().cloned());if let Some(model)=selected{fallback.push(format!("{}:{}",id,model.id));if required.iter().all(|cap|cap=="chat"||model.capabilities.contains(cap)){return Ok(RouteDecision{provider_id:id.into(),model:model.id,reason:if private{"Private/local-only routing".into()}else{format!("Capability match: {}",required.join(", "))},required_capabilities:required,fallback_chain:fallback})}}}
  Err(ObError::RequiresConfiguration(format!("No enabled model satisfies: {}. Configure Ollama, Gemini, or an OpenAI-compatible endpoint.",required.join(", "))))
 }
}
