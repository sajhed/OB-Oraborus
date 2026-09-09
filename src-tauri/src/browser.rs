use crate::error::{ObError,ObResult};
use serde::{Deserialize,Serialize};
use serde_json::Value;
use std::{path::Path,process::Stdio,time::Duration};
use tokio::{io::AsyncWriteExt,process::Command};
use tokio_util::sync::CancellationToken;

#[derive(Debug,Clone,Serialize,Deserialize)] #[serde(rename_all="camelCase")]
pub struct BrowserRequest { pub profile_dir:String, pub headless:bool, pub actions:Vec<BrowserAction>, pub timeout_ms:u64 }
#[derive(Debug,Clone,Serialize,Deserialize)] #[serde(tag="type",rename_all="snake_case")]
pub enum BrowserAction { Navigate{url:String}, Read{selector:Option<String>}, Click{selector:String,expected_text:Option<String>}, Fill{selector:String,value:String,secret:bool}, Press{key:String}, WaitFor{selector:String}, Screenshot{path:String}, Download{selector:String,directory:String} }
#[derive(Debug,Clone,Serialize,Deserialize)] #[serde(rename_all="camelCase")]
pub struct BrowserResult { pub state:String, pub final_url:Option<String>, pub title:Option<String>, pub text:Option<String>, pub screenshot:Option<String>, pub download:Option<String>, pub verification:Value, pub error:Option<String> }

pub async fn run(runner:&Path,request:&BrowserRequest,cancellation:CancellationToken)->ObResult<BrowserResult>{
 if !runner.exists(){return Err(ObError::RequiresConfiguration("Playwright runner was not bundled".into()))}
 for action in &request.actions{if let BrowserAction::Navigate{url}=action{let parsed=url::Url::parse(url).map_err(|_|ObError::Validation("Browser URL is invalid".into()))?;if !matches!(parsed.scheme(),"http"|"https"){return Err(ObError::SecurityBlocked("Only HTTP and HTTPS navigation is allowed".into()))}}}
 let mut child=Command::new("node").arg(runner).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).kill_on_drop(true).spawn().map_err(|_|ObError::RequiresConfiguration("Node.js and Playwright are required for browser automation".into()))?;
 if let Some(mut stdin)=child.stdin.take(){stdin.write_all(serde_json::to_string(request)?.as_bytes()).await?;stdin.shutdown().await?;}
 let output=tokio::select!{_ = cancellation.cancelled()=>{let _=child.kill().await;return Err(ObError::Cancelled)},value=tokio::time::timeout(Duration::from_millis(request.timeout_ms.clamp(1_000,300_000)),child.wait_with_output())=>value.map_err(|_|ObError::Unavailable("Browser workflow timed out".into()))??};
 if !output.status.success(){return Err(ObError::Unavailable(format!("Browser runner failed: {}",String::from_utf8_lossy(&output.stderr))))}
 serde_json::from_slice(&output.stdout).map_err(Into::into)
}
