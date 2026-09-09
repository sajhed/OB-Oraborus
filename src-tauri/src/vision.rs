use crate::error::{ObError,ObResult};
use base64::Engine;
use serde::{Deserialize,Serialize};
use sha2::{Digest,Sha256};
use std::{path::Path,process::Command};

#[derive(Debug,Clone,Serialize,Deserialize)] #[serde(rename_all="camelCase")]
pub struct CaptureEvidence { pub path:String, pub width:u32, pub height:u32, pub sha256:String, pub captured_at:String }
#[derive(Debug,Clone,Serialize,Deserialize)] #[serde(rename_all="camelCase")]
pub struct OcrOutput { pub engine:String, pub text:String, pub source_hash:String, pub confidence:Option<f32> }

pub fn capture(display_index:usize,destination:&Path)->ObResult<CaptureEvidence>{
 let screens=screenshots::Screen::all().map_err(|error|ObError::Unavailable(format!("Screen enumeration failed: {error}")))?;
 let screen=screens.get(display_index).ok_or_else(||ObError::Validation("Display index does not exist".into()))?;
 let image=screen.capture().map_err(|error|ObError::Unavailable(format!("Screen capture failed: {error}")))?;
 image.save(destination).map_err(|error|ObError::Io(std::io::Error::other(error)))?;
 let bytes=std::fs::read(destination)?; let digest=Sha256::digest(&bytes).iter().map(|byte|format!("{byte:02x}")).collect();
 Ok(CaptureEvidence{path:destination.to_string_lossy().into_owned(),width:image.width(),height:image.height(),sha256:digest,captured_at:chrono::Utc::now().to_rfc3339()})
}
pub fn ocr(path:&Path,language:&str)->ObResult<OcrOutput>{if which::which("tesseract").is_err(){return Err(ObError::RequiresConfiguration("Install Tesseract and the requested language pack".into()))}let output=Command::new("tesseract").arg(path).arg("stdout").args(["-l",language]).output()?;if !output.status.success(){return Err(ObError::Unavailable(String::from_utf8_lossy(&output.stderr).into_owned()))}let hash=Sha256::digest(std::fs::read(path)?).iter().map(|byte|format!("{byte:02x}")).collect();Ok(OcrOutput{engine:"Tesseract".into(),text:String::from_utf8_lossy(&output.stdout).into_owned(),source_hash:hash,confidence:None})}
pub fn image_base64(path:&Path)->ObResult<(String,String)>{let bytes=std::fs::read(path)?;let extension=path.extension().and_then(|value|value.to_str()).unwrap_or("").to_lowercase();let mime=match extension.as_str(){"png"=>"image/png","jpg"|"jpeg"=>"image/jpeg","webp"=>"image/webp",_=>return Err(ObError::Validation("Unsupported vision image type".into()))};Ok((mime.into(),base64::engine::general_purpose::STANDARD.encode(bytes)))}
