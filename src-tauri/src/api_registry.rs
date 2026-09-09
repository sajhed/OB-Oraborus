use crate::{db::Database,error::{ObError,ObResult}};
use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize,Serialize};

#[derive(Debug,Clone,Serialize,Deserialize)] #[serde(rename_all="camelCase")]
pub struct ApiEntry { pub api_id:String, pub name:String, pub category:String, pub base_url:String, pub authentication:String, pub https:bool, pub cors:String, pub status:String, pub last_checked:Option<String>, pub rate_limit:String, pub documentation_url:String, pub adapter_name:Option<String>, pub capabilities:Vec<String>, pub enabled:bool }
#[derive(Clone)] pub struct ApiRegistry { db:Database, client:reqwest::Client }
impl ApiRegistry {
 pub fn load(db:Database)->ObResult<Self>{
  let entries:Vec<ApiEntry>=serde_json::from_str(include_str!("../../../integrations/public-api-registry.json"))?;
  for entry in entries { db.execute(|connection|{connection.execute("INSERT INTO api_registry(api_id,name,category,base_url,authentication,https,cors,status,last_checked,rate_limit,documentation_url,adapter_name,capabilities_json,enabled) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14) ON CONFLICT(api_id) DO UPDATE SET name=excluded.name,category=excluded.category,base_url=excluded.base_url,documentation_url=excluded.documentation_url,adapter_name=excluded.adapter_name,capabilities_json=excluded.capabilities_json",params![entry.api_id,entry.name,entry.category,entry.base_url,entry.authentication,entry.https as i32,entry.cors,entry.status,entry.last_checked,entry.rate_limit,entry.documentation_url,entry.adapter_name,serde_json::to_string(&entry.capabilities)?,entry.enabled as i32])?;Ok(())})?; }
  Ok(Self{db,client:reqwest::Client::builder().timeout(std::time::Duration::from_secs(15)).build().expect("HTTP client")})
 }
 pub fn list(&self)->ObResult<Vec<ApiEntry>>{self.db.execute(|connection|{let mut statement=connection.prepare("SELECT api_id,name,category,base_url,authentication,https,cors,status,last_checked,rate_limit,documentation_url,adapter_name,capabilities_json,enabled FROM api_registry ORDER BY category,name")?;let rows=statement.query_map([],|row|Ok(ApiEntry{api_id:row.get(0)?,name:row.get(1)?,category:row.get(2)?,base_url:row.get(3)?,authentication:row.get(4)?,https:row.get::<_,i64>(5)?!=0,cors:row.get(6)?,status:row.get(7)?,last_checked:row.get(8)?,rate_limit:row.get(9)?,documentation_url:row.get(10)?,adapter_name:row.get(11)?,capabilities:serde_json::from_str(&row.get::<_,String>(12)?).unwrap_or_default(),enabled:row.get::<_,i64>(13)?!=0}))?;Ok(rows.flatten().collect())})}
 pub async fn health(&self,api_id:&str)->ObResult<String>{let entry=self.list()?.into_iter().find(|entry|entry.api_id==api_id).ok_or_else(||ObError::Validation("API entry was not found".into()))?;let started=std::time::Instant::now();let result=self.client.get(&entry.base_url).send().await;let status=match result{Ok(response) if response.status().as_u16()==429=>"RATE LIMITED",Ok(response) if matches!(response.status().as_u16(),401|403)=>"AUTH REQUIRED",Ok(response) if response.status().is_success()&&started.elapsed().as_secs()<3=>"ONLINE",Ok(response) if response.status().is_success()=>"SLOW",Ok(_)=>"OFFLINE",Err(_)=>"OFFLINE"};self.db.execute(|connection|{connection.execute("UPDATE api_registry SET status=?2,last_checked=?3 WHERE api_id=?1",params![api_id,status,Utc::now().to_rfc3339()])?;Ok(())})?;Ok(status.into())}
}
