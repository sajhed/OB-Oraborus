use crate::error::{ObError,ObResult};
use chrono::{DateTime,Utc};
use regex::Regex;
use serde::{Deserialize,Serialize};
use sha2::{Digest,Sha256};
use url::Url;

#[derive(Debug,Clone,Serialize,Deserialize)] #[serde(rename_all="camelCase")]
pub struct ResearchSource { pub title:String, pub url:String, pub retrieved_at:DateTime<Utc>, pub key_finding:String, pub confidence:f32, pub content_hash:String, pub text:String }
#[derive(Clone)] pub struct ResearchEngine { client:reqwest::Client }
impl ResearchEngine {
 pub fn new()->Self{Self{client:reqwest::Client::builder().timeout(std::time::Duration::from_secs(30)).user_agent("OB-Oraborus/0.1 research reader").build().expect("HTTP client")}}
 pub async fn read(&self,url:&str)->ObResult<ResearchSource>{
  let parsed=Url::parse(url).map_err(|_|ObError::Validation("Research URL is invalid".into()))?; if !matches!(parsed.scheme(),"http"|"https"){return Err(ObError::SecurityBlocked("Research reader allows only HTTP and HTTPS".into()))}
  let response=self.client.get(parsed).send().await?.error_for_status()?; let canonical=response.url().to_string(); let html=response.text().await?;
  if html.len()>10_000_000{return Err(ObError::Validation("Research source exceeds the 10 MB text limit".into()))}
  let title=Regex::new(r"(?is)<title[^>]*>(.*?)</title>").expect("title pattern").captures(&html).and_then(|capture|capture.get(1)).map(|value|decode(value.as_str())).unwrap_or_else(||canonical.clone());
  let stripped=Regex::new(r"(?is)<(script|style|nav|footer)[^>]*>.*?</(script|style|nav|footer)>|<[^>]+>").expect("HTML pattern").replace_all(&html," ");
  let text=Regex::new(r"\s+").expect("space pattern").replace_all(&decode(&stripped)," ").trim().chars().take(1_000_000).collect::<String>();
  let finding=text.chars().take(420).collect::<String>(); let hash=Sha256::digest(html.as_bytes()).iter().map(|byte|format!("{byte:02x}")).collect();
  Ok(ResearchSource{title,url:canonical,retrieved_at:Utc::now(),key_finding:finding,confidence:.5,content_hash:hash,text})
 }
}
fn decode(value:&str)->String{value.replace("&amp;","&").replace("&lt;","<").replace("&gt;",">").replace("&quot;","\"").replace("&#39;","'")}
