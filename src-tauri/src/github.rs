use crate::{error::{ObError,ObResult},secrets::SecretVault};
use serde::{Deserialize,Serialize};
use serde_json::Value;

#[derive(Clone)] pub struct GitHubClient { client:reqwest::Client, vault:SecretVault }
#[derive(Debug,Clone,Serialize,Deserialize)] #[serde(rename_all="camelCase")]
pub struct GitHubResponse { pub endpoint:String, pub status:u16, pub retrieved_at:String, pub data:Value }
impl GitHubClient {
 pub fn new(vault:SecretVault)->Self{Self{client:reqwest::Client::builder().user_agent("OB-Oraborus/0.1").timeout(std::time::Duration::from_secs(30)).build().expect("HTTP client"),vault}}
 pub async fn get(&self,path:&str)->ObResult<GitHubResponse>{
  if !path.starts_with('/')||path.contains(".."){return Err(ObError::Validation("GitHub API path is invalid".into()))}
  let endpoint = "https://api.github.com".to_string() + path;
  let mut request=self.client.get(&endpoint).header("Accept","application/vnd.github+json").header("X-GitHub-Api-Version","2022-11-28");
  if let Ok(token)=self.vault.get("github"){request=request.bearer_auth(token.as_str())}
  let response=request.send().await?; let status=response.status(); let data:Value=response.error_for_status()?.json().await?;
  Ok(GitHubResponse{endpoint,status:status.as_u16(),retrieved_at:chrono::Utc::now().to_rfc3339(),data})
 }
}
