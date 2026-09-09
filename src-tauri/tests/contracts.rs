use std::{collections::HashSet,fs,path::PathBuf};
use serde_json::Value;

fn root()->PathBuf{PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().expect("project root").to_path_buf()}
#[test]
fn registry_has_thirty_unique_agents(){let value:Value=serde_json::from_str(&fs::read_to_string(root().join("agents/registry.json")).expect("agent registry")).expect("valid JSON");let agents=value.as_array().expect("agent array");let ids:HashSet<_>=agents.iter().filter_map(|agent|agent.get("id").and_then(Value::as_str)).collect();assert_eq!(agents.len(),30);assert_eq!(ids.len(),30);}
#[test]
fn every_agent_tool_is_registered(){let agents:Value=serde_json::from_str(&fs::read_to_string(root().join("agents/registry.json")).expect("agents")).expect("agent JSON");let tools:Value=serde_json::from_str(&fs::read_to_string(root().join("tools/registry.json")).expect("tools")).expect("tool JSON");let names:HashSet<_>=tools.as_array().expect("tool array").iter().filter_map(|tool|tool.get("name").and_then(Value::as_str)).collect();for agent in agents.as_array().expect("agent array"){for tool in agent.get("tools").and_then(Value::as_array).expect("agent tools"){assert!(names.contains(tool.as_str().expect("tool name")));}}}
#[test]
fn migration_contains_required_domains(){let migration=fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("migrations/0001_init.sql")).expect("migration");for table in ["users","providers","messages","memories","memory_embeddings","documents","agents","agent_runs","tools","tool_runs","tasks","task_steps","automations","permissions","audit_logs","api_registry","research_sources","knowledge_entities","knowledge_relations"]{assert!(migration.contains(&format!("CREATE TABLE IF NOT EXISTS {table}")),"missing {table}");}}
