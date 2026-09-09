use crate::error::{ObError,ObResult};
use serde::{Deserialize,Serialize};
use std::path::Path;

#[derive(Debug,Clone,Serialize,Deserialize)] #[serde(rename_all="camelCase")]
pub struct PluginManifest { pub id:String, pub name:String, pub version:String, pub author:String, pub entry:String, pub capabilities:Vec<String>, pub permissions:Vec<String>, pub dependencies:Vec<String>, pub security_requirements:Vec<String>, pub signature:Option<String> }
pub fn load_manifest(path:&Path)->ObResult<PluginManifest>{let bytes=std::fs::read(path)?;if bytes.len()>1024*1024{return Err(ObError::Validation("Plugin manifest exceeds 1 MB".into()))}let manifest:PluginManifest=serde_json::from_slice(&bytes)?;if manifest.id.trim().is_empty()||manifest.version.trim().is_empty()||manifest.entry.contains("..")||Path::new(&manifest.entry).is_absolute(){return Err(ObError::SecurityBlocked("Plugin manifest contains an unsafe identifier or entry path".into()))}Ok(manifest)}
