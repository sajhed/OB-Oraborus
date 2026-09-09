use crate::{error::{ObError,ObResult},security::SecurityGate};
use chrono::{DateTime,Utc};
use serde::{Deserialize,Serialize};
use sha2::{Digest,Sha256};
use std::{fs,path::Path};
use walkdir::WalkDir;

#[derive(Debug,Clone,Serialize,Deserialize)] #[serde(rename_all="camelCase")]
pub struct FileMatch { pub path:String, pub name:String, pub size_bytes:u64, pub modified_at:Option<DateTime<Utc>>, pub kind:String, pub matched_by:Vec<String> }
#[derive(Debug,Clone,Serialize,Deserialize)] #[serde(rename_all="camelCase")]
pub struct FileEvidence { pub path:String, pub exists:bool, pub size_bytes:Option<u64>, pub sha256:Option<String> }

pub fn search(gate:&SecurityGate,root:&Path,query:&str,limit:usize)->ObResult<Vec<FileMatch>> {
 let root=gate.approved_path(root,false)?;
 let terms:Vec<String>=query.to_lowercase().split_whitespace().map(str::to_string).collect();
 let mut output=Vec::new();
 for entry in WalkDir::new(root).follow_links(false).max_depth(12).into_iter().filter_map(Result::ok) {
  if output.len()>=limit.clamp(1,500){break}
  let name=entry.file_name().to_string_lossy().into_owned(); let lower=name.to_lowercase();
  if !terms.is_empty() && !terms.iter().all(|term|lower.contains(term)){continue}
  let metadata=match entry.metadata(){Ok(value)=>value,Err(_)=>continue};
  output.push(FileMatch{path:entry.path().to_string_lossy().into_owned(),name,size_bytes:metadata.len(),modified_at:metadata.modified().ok().map(DateTime::<Utc>::from),kind:if metadata.is_dir(){"folder".into()}else{"file".into()},matched_by:vec!["filename".into()]});
 }
 Ok(output)
}
pub fn create_directory(gate:&SecurityGate,path:&Path)->ObResult<FileEvidence>{let path=gate.approved_path(path,true)?;if path.exists(){return Err(ObError::Validation("Destination already exists".into()))}fs::create_dir(&path)?;verify_present(&path,None)}
pub fn write_text(gate:&SecurityGate,path:&Path,content:&str)->ObResult<FileEvidence>{let path=gate.approved_path(path,true)?;if path.exists(){return Err(ObError::Validation("Destination already exists; overwrite requires explicit approval".into()))}fs::write(&path,content.as_bytes())?;verify_present(&path,Some(content.len() as u64))}
pub fn copy(gate:&SecurityGate,source:&Path,destination:&Path)->ObResult<FileEvidence>{let source=gate.approved_path(source,false)?;let destination=gate.approved_path(destination,true)?;if destination.exists(){return Err(ObError::Validation("Destination already exists; overwrite requires explicit approval".into()))}let source_hash=sha256(&source)?;fs::copy(&source,&destination)?;let result=evidence(&destination)?;if result.sha256.as_deref()!=Some(source_hash.as_str()){let _=fs::remove_file(&destination);return Err(ObError::Internal("Copied file hash did not match source".into()))}Ok(result)}
pub fn move_path(gate:&SecurityGate,source:&Path,destination:&Path)->ObResult<FileEvidence>{let source=gate.approved_path(source,false)?;let destination=gate.approved_path(destination,true)?;if destination.exists(){return Err(ObError::Validation("Destination already exists; choose rename, skip, or approve overwrite".into()))}fs::rename(&source,&destination)?;let result=evidence(&destination)?;if source.exists()||!result.exists{return Err(ObError::Internal("Move verification failed".into()))}Ok(result)}
pub fn delete_path(gate:&SecurityGate,path:&Path)->ObResult<FileEvidence>{let path=gate.approved_path(path,false)?;if path.is_dir(){fs::remove_dir_all(&path)?}else{fs::remove_file(&path)?}let result=evidence(&path)?;if result.exists{return Err(ObError::Internal("Delete verification failed".into()))}Ok(result)}
pub fn evidence(path:&Path)->ObResult<FileEvidence>{if !path.exists(){return Ok(FileEvidence{path:path.to_string_lossy().into_owned(),exists:false,size_bytes:None,sha256:None})}let metadata=fs::metadata(path)?;Ok(FileEvidence{path:path.to_string_lossy().into_owned(),exists:true,size_bytes:metadata.is_file().then_some(metadata.len()),sha256:if metadata.is_file(){Some(sha256(path)?)}else{None}})}
fn verify_present(path:&Path,size:Option<u64>)->ObResult<FileEvidence>{let result=evidence(path)?;if !result.exists||size.is_some()&&result.size_bytes!=size{return Err(ObError::Internal("Filesystem verification failed".into()))}Ok(result)}
fn sha256(path:&Path)->ObResult<String>{let digest=Sha256::digest(fs::read(path)?);Ok(digest.iter().map(|byte|format!("{byte:02x}")).collect())}
