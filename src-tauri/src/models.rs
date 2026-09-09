use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CapabilityState { Available, NotConnected, Unavailable, RequiresConfiguration, Disabled }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RiskLevel { ReadOnly, Safe, Sensitive, Destructive, Critical }

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetryPolicy { pub max_attempts: u8, pub backoff_ms: u64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentDefinition {
    pub id: String,
    pub name: String,
    pub purpose: String,
    pub system_instructions: String,
    pub tools: Vec<String>,
    pub permissions: Vec<RiskLevel>,
    pub model_preference: Vec<String>,
    pub input_schema: Value,
    pub output_schema: Value,
    pub timeout_ms: u64,
    pub retry: RetryPolicy,
    pub verification: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentDescriptor {
    #[serde(flatten)] pub definition: AgentDefinition,
    pub state: String,
    pub current_task: Option<String>,
    pub parent_agent: Option<String>,
    pub collaborators: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderStatus {
    pub id: String,
    pub label: String,
    pub state: CapabilityState,
    pub active_model: Option<String>,
    pub capabilities: Vec<String>,
    pub latency_ms: Option<u64>,
    pub detail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GpuSnapshot {
    pub name: String,
    pub utilization_percent: Option<f32>,
    pub memory_used_bytes: Option<u64>,
    pub memory_total_bytes: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemSnapshot {
    pub measured_at: DateTime<Utc>,
    pub cpu_percent: Option<f32>,
    pub memory_used_bytes: Option<u64>,
    pub memory_total_bytes: Option<u64>,
    pub disk_used_bytes: Option<u64>,
    pub disk_total_bytes: Option<u64>,
    pub battery_percent: Option<f32>,
    pub gpu: Vec<GpuSnapshot>,
    pub network_received_bytes: Option<u64>,
    pub network_transmitted_bytes: Option<u64>,
    pub sensor_notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskEvent {
    pub id: String,
    pub at: DateTime<Utc>,
    pub kind: String,
    pub label: String,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskProgress { pub completed: u32, pub total: u32 }

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskSummary {
    pub id: String,
    pub title: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub progress: Option<TaskProgress>,
    pub events: Vec<TaskEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivacyState {
    pub microphone: bool,
    pub camera: bool,
    pub screen_capture: bool,
    pub filesystem: bool,
    pub browser: bool,
    pub network: bool,
    pub memory: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSnapshot {
    pub connectivity: String,
    pub ora_state: String,
    pub local_only: bool,
    pub autonomy_level: u8,
    pub providers: Vec<ProviderStatus>,
    pub agents: Vec<AgentDescriptor>,
    pub active_task: Option<TaskSummary>,
    pub recent_tasks: Vec<TaskSummary>,
    pub system: Option<SystemSnapshot>,
    pub privacy: PrivacyState,
    pub unread_notifications: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandRequest { pub text: String, #[serde(default)] pub attachments: Vec<String> }

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApprovalRequest {
    pub id: String,
    pub action: String,
    pub risk: RiskLevel,
    pub summary: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Citation { pub title: String, pub url: String, pub retrieved_at: DateTime<Utc> }

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandResponse {
    pub task_id: String,
    pub state: String,
    pub message: String,
    pub provider: Option<String>,
    pub model: Option<String>,
    pub route_reason: Option<String>,
    pub approval: Option<ApprovalRequest>,
    pub citations: Vec<Citation>,
}

impl CommandResponse {
    pub fn completed(task_id: impl Into<String>, message: impl Into<String>) -> Self {
        Self { task_id: task_id.into(), state: "COMPLETED".into(), message: message.into(), provider: None, model: None, route_reason: None, approval: None, citations: vec![] }
    }
    pub fn failed(task_id: impl Into<String>, message: impl Into<String>) -> Self {
        Self { task_id: task_id.into(), state: "FAILED".into(), message: message.into(), provider: None, model: None, route_reason: None, approval: None, citations: vec![] }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
    pub output_schema: Value,
    pub permission: RiskLevel,
    pub timeout_ms: u64,
    pub agent_access: Vec<String>,
    pub verification_method: String,
    pub implementation: String,
    pub enabled_by_default: bool,
}
