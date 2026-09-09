use crate::{db::Database,error::{ObError,ObResult},models::RiskLevel};
use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize,Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug,Clone,Serialize,Deserialize)] #[serde(rename_all="camelCase")]
pub struct AutomationDefinition { pub id:String, pub name:String, pub trigger:Value, pub steps:Vec<Value>, pub permission_scope:Value, pub risk:RiskLevel, pub enabled:bool, pub maximum_actions:u32, pub created_at:String, pub updated_at:String }
#[derive(Clone)] pub struct AutomationEngine { db:Database }
impl AutomationEngine {
 pub fn new(db:Database)->Self{Self{db}}
 pub fn create(&self,name:&str,trigger:Value,steps:Vec<Value>,permission_scope:Value,risk:RiskLevel)->ObResult<AutomationDefinition>{if name.trim().is_empty()||steps.is_empty(){return Err(ObError::Validation("Automation requires a name and at least one step".into()))}if steps.len()>100{return Err(ObError::Validation("Automation exceeds the 100-action hard limit".into()))}let id=Uuid::new_v4().to_string();let now=Utc::now().to_rfc3339();self.db.execute(|connection|{connection.execute("INSERT INTO automations(id,name,trigger_json,steps_json,permission_scope_json,risk,enabled,created_at,updated_at) VALUES(?1,?2,?3,?4,?5,?6,0,?7,?7)",params![id,name,serde_json::to_string(&trigger)?,serde_json::to_string(&steps)?,serde_json::to_string(&permission_scope)?,format!("{risk:?}").to_uppercase(),now])?;Ok(())})?;Ok(AutomationDefinition{id,name:name.into(),trigger,steps,permission_scope,risk,enabled:false,maximum_actions:100,created_at:now.clone(),updated_at:now})}
 pub fn disable_all(&self)->ObResult<usize>{self.db.execute(|connection|Ok(connection.execute("UPDATE automations SET enabled=0,updated_at=?1 WHERE enabled=1",[Utc::now().to_rfc3339()])?))}
}
