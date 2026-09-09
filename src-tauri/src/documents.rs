use crate::error::{ObError,ObResult};
use quick_xml::{events::Event,Reader};
use serde::{Deserialize,Serialize};
use sha2::{Digest,Sha256};
use std::{fs::File,io::Read,path::Path,process::Command};
use zip::ZipArchive;

#[derive(Debug,Clone,Serialize,Deserialize)] #[serde(rename_all="camelCase")]
pub struct ExtractedDocument { pub path:String, pub parser:String, pub text:String, pub source_hash:String, pub bytes_read:u64, pub warnings:Vec<String> }
pub fn extract(path:&Path)->ObResult<ExtractedDocument>{
 let metadata=std::fs::metadata(path)?; if metadata.len()>100*1024*1024{return Err(ObError::Validation("Document exceeds the 100 MB local parsing limit".into()))}
 let bytes=std::fs::read(path)?; let hash=Sha256::digest(&bytes).iter().map(|byte|format!("{byte:02x}")).collect();
 let extension=path.extension().and_then(|value|value.to_str()).unwrap_or("").to_lowercase();
 let (parser,text,warnings)=match extension.as_str(){
  "txt"|"md"|"json"|"csv"|"tsv"|"rs"|"ts"|"tsx"|"js"|"py"|"toml"|"yaml"|"yml"=>("UTF-8 local reader".into(),String::from_utf8(bytes).map_err(|_|ObError::Validation("Text file is not valid UTF-8".into()))?,vec![]),
  "docx"=>("DOCX Open XML".into(),extract_docx(path)?,vec![]),
  "pdf"=>{if which::which("pdftotext").is_err(){return Err(ObError::RequiresConfiguration("Install Poppler pdftotext for local PDF extraction".into()))}let output=Command::new("pdftotext").arg("-layout").arg(path).arg("-").output()?;if !output.status.success(){return Err(ObError::Unavailable(String::from_utf8_lossy(&output.stderr).into_owned()))}("Poppler pdftotext".into(),String::from_utf8_lossy(&output.stdout).into_owned(),vec![])},
  _=>return Err(ObError::Unavailable(format!("No local parser is configured for .{extension}")))
 };
 Ok(ExtractedDocument{path:path.to_string_lossy().into_owned(),parser,text,source_hash:hash,bytes_read:metadata.len(),warnings})
}
fn extract_docx(path:&Path)->ObResult<String>{let file=File::open(path)?;let mut archive=ZipArchive::new(file).map_err(|error|ObError::Validation(format!("DOCX archive is invalid: {error}")))?;let mut xml=String::new();archive.by_name("word/document.xml").map_err(|_|ObError::Validation("DOCX document.xml is missing".into()))?.read_to_string(&mut xml)?;let mut reader=Reader::from_str(&xml);let mut output=String::new();loop{match reader.read_event(){Ok(Event::Text(value))=>{output.push_str(&value.unescape().map_err(|error|ObError::Validation(format!("DOCX XML error: {error}")))?);output.push(' ')},Ok(Event::End(value)) if value.name().as_ref()==b"w:p"=>output.push('\n'),Ok(Event::Eof)=>break,Err(error)=>return Err(ObError::Validation(format!("DOCX XML error: {error}"))),_=>{}}}Ok(output)}
