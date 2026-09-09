use crate::{agents::AgentRegistry,api_registry::ApiRegistry,audit::AuditLog,automation::AutomationEngine,db::Database,error::ObResult,github::GitHubClient,memory::MemoryStore,model_router::ModelRouter,providers::ProviderManager,research::ResearchEngine,secrets::SecretVault,security::SecurityGate,tool_registry::ToolRegistry};
use parking_lot::RwLock;
use std::{path::PathBuf,sync::Arc};
use tokio_util::sync::CancellationToken;

#[derive(Clone)] pub struct AppState { pub db:Database, pub vault:SecretVault, pub audit:AuditLog, pub security:SecurityGate, pub providers:ProviderManager, pub router:ModelRouter, pub agents:AgentRegistry, pub tools:ToolRegistry, pub memory:MemoryStore, pub research:ResearchEngine, pub github:GitHubClient, pub api_registry:ApiRegistry, pub automations:AutomationEngine, pub cancellation:Arc<RwLock<CancellationToken>>, pub data_dir:PathBuf, pub resource_dir:PathBuf }
impl AppState {
 pub fn initialize(data_dir:PathBuf,resource_dir:PathBuf)->ObResult<Self>{std::fs::create_dir_all(&data_dir)?;let db=Database::open(&data_dir.join("ob-oraborus.db"))?;let vault=SecretVault::default();let audit=AuditLog::new(db.clone());let security=SecurityGate::new(db.clone(),audit.clone());let providers=ProviderManager::new(db.clone(),vault.clone());let router=ModelRouter::new(db.clone(),providers.clone());let agents=AgentRegistry::load(db.clone())?;let tools=ToolRegistry::load(&db)?;let memory=MemoryStore::new(db.clone());let research=ResearchEngine::new();let github=GitHubClient::new(vault.clone());let api_registry=ApiRegistry::load(db.clone())?;let automations=AutomationEngine::new(db.clone());Ok(Self{db,vault,audit,security,providers,router,agents,tools,memory,research,github,api_registry,automations,cancellation:Arc::new(RwLock::new(CancellationToken::new())),data_dir,resource_dir})}
 pub fn token(&self)->CancellationToken{self.cancellation.read().clone()}
 pub fn stop_all(&self){self.cancellation.read().cancel();*self.cancellation.write()=CancellationToken::new();}
 pub fn browser_runner(&self)->PathBuf{self.resource_dir.join("browser").join("runner.mjs")}
}
