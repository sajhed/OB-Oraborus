use crate::{error::{ObError,ObResult},security::SecurityGate};
use serde::{Deserialize,Serialize};
use std::{path::Path,process::Stdio,time::Duration};
use tokio::{io::AsyncWriteExt,process::Command};
use tokio_util::sync::CancellationToken;

#[derive(Debug,Clone,Serialize,Deserialize)] #[serde(rename_all="camelCase")]
pub struct TerminalOutput { pub command_preview:String, pub exit_code:Option<i32>, pub stdout:String, pub stderr:String, pub timed_out:bool, pub cancelled:bool }

pub async fn execute(command:&str,working_directory:Option<&Path>,timeout:Duration,cancellation:CancellationToken)->ObResult<TerminalOutput>{
 let _risk=SecurityGate::validate_terminal(command)?;
 let mut process=Command::new("powershell.exe");
 process.args(["-NoLogo","-NoProfile","-NonInteractive","-ExecutionPolicy","RemoteSigned","-Command","-"]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).kill_on_drop(true);
 if let Some(dir)=working_directory{process.current_dir(dir);}
 let mut child=process.spawn()?;
 if let Some(mut stdin)=child.stdin.take(){stdin.write_all(command.as_bytes()).await?;stdin.shutdown().await?;}
 let output=tokio::select!{
  _=cancellation.cancelled()=>{let _=child.kill().await;return Ok(TerminalOutput{command_preview:redact_command(command),exit_code:None,stdout:String::new(),stderr:String::new(),timed_out:false,cancelled:true})},
  value=tokio::time::timeout(timeout,child.wait_with_output())=>match value{Ok(result)=>result?,Err(_)=>return Ok(TerminalOutput{command_preview:redact_command(command),exit_code:None,stdout:String::new(),stderr:"Process exceeded its timeout and was terminated.".into(),timed_out:true,cancelled:false})}
 };
 Ok(TerminalOutput{command_preview:redact_command(command),exit_code:output.status.code(),stdout:bounded(&output.stdout),stderr:bounded(&output.stderr),timed_out:false,cancelled:false})
}
fn bounded(bytes:&[u8])->String{const MAX:usize=1_000_000;String::from_utf8_lossy(&bytes[..bytes.len().min(MAX)]).into_owned()}
fn redact_command(command:&str)->String{let pattern=regex::Regex::new(r"(?i)(api.?key|token|password|secret)\s*[:=]\s*[^\s;]+").expect("command redaction");pattern.replace_all(command,"$1=[REDACTED]").into_owned()}
