use crate::{error::{ObError,ObResult},system_monitor};
use serde::{Deserialize,Serialize};
use std::{collections::HashMap,path::Path,process::Command,time::Duration};

#[derive(Debug,Clone,Serialize,Deserialize)] #[serde(rename_all="camelCase")]
pub struct LaunchEvidence { pub requested:String, pub executable:String, pub process_detected:bool, pub process_name:Option<String> }
fn app_map()->HashMap<&'static str,&'static str>{HashMap::from([("notepad","notepad.exe"),("chrome","chrome.exe"),("google chrome","chrome.exe"),("edge","msedge.exe"),("microsoft edge","msedge.exe"),("vs code","code.cmd"),("visual studio code","code.cmd"),("calculator","calc.exe"),("explorer","explorer.exe"),("file explorer","explorer.exe"),("spotify","spotify.exe"),("powershell","powershell.exe")])}
pub fn launch(requested:&str)->ObResult<LaunchEvidence>{
 let normalized=requested.trim().trim_matches(|c:char|matches!(c,'.'|','|'!'|'?')).to_lowercase();
 let executable=app_map().get(normalized.as_str()).ok_or_else(||ObError::RequiresConfiguration(format!("'{requested}' is not in the safe application registry. Add an explicit executable in Settings → Tools.")))?.to_string();
 Command::new(&executable).spawn().map_err(|error|ObError::Unavailable(format!("Application launch failed: {error}")))?;
 std::thread::sleep(Duration::from_millis(500));
 let process_token=executable.trim_end_matches(".exe").trim_end_matches(".cmd").to_lowercase();
 let found=system_monitor::processes(200).into_iter().find(|process|process.name.to_lowercase().contains(&process_token));
 Ok(LaunchEvidence{requested:requested.into(),executable,process_detected:found.is_some(),process_name:found.map(|process|process.name)})
}
pub fn open_folder(path:&Path)->ObResult<LaunchEvidence>{if !path.is_dir(){return Err(ObError::Validation("Folder does not exist".into()))}Command::new("explorer.exe").arg(path).spawn()?;Ok(LaunchEvidence{requested:path.to_string_lossy().into_owned(),executable:"explorer.exe".into(),process_detected:true,process_name:Some("explorer.exe".into())})}
