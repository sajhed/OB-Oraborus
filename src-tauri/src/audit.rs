use crate::{db::Database, error::ObResult, models::RiskLevel};
use chrono::Utc;
use regex::Regex;
use rusqlite::params;
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone)] pub struct AuditLog { db: Database }
impl AuditLog {
    pub fn new(db: Database) -> Self { Self { db } }
    pub fn record(&self, category: &str, action: &str, actor: &str, risk: &RiskLevel, status: &str, summary: &str, details: Value) -> ObResult<()> {
        let redacted = redact(details);
        self.db.execute(|connection| { connection.execute("INSERT INTO audit_logs(id,timestamp,category,action,actor,risk,status,summary,details_redacted_json) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9)", params![Uuid::new_v4().to_string(),Utc::now().to_rfc3339(),category,action,actor,format!("{risk:?}").to_uppercase(),status,summary,serde_json::to_string(&redacted)?])?; Ok(()) })
    }
}

fn redact(value: Value) -> Value {
    let sensitive_key = Regex::new("(?i)(api.?key|authorization|password|secret|token|credential|cookie)").expect("redaction pattern");
    match value {
        Value::Object(map) => Value::Object(map.into_iter().map(|(key,value)| { if sensitive_key.is_match(&key) { (key,Value::String("[REDACTED]".into())) } else { (key,redact(value)) } }).collect()),
        Value::Array(values) => Value::Array(values.into_iter().map(redact).collect()),
        other => other,
    }
}

#[cfg(test)] mod tests { use super::*; #[test] fn secrets_are_redacted() { let value=redact(serde_json::json!({"apiKey":"abc","nested":{"token":"xyz"},"safe":3})); assert_eq!(value["apiKey"],"[REDACTED]"); assert_eq!(value["safe"],3); } }
