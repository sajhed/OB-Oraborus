use crate::{error::{ObError, ObResult}, models::{PrivacyState, TaskEvent, TaskSummary}};
use chrono::{DateTime, Utc};
use parking_lot::Mutex;
use rusqlite::{params, Connection, OptionalExtension};
use serde_json::Value;
use std::{path::Path, sync::Arc};
use uuid::Uuid;

#[derive(Clone)]
pub struct Database { inner: Arc<Mutex<Connection>> }

impl Database {
    pub fn open(path: &Path) -> ObResult<Self> {
        if let Some(parent) = path.parent() { std::fs::create_dir_all(parent)?; }
        let connection = Connection::open(path)?;
        connection.pragma_update(None, "journal_mode", "WAL")?;
        connection.pragma_update(None, "foreign_keys", "ON")?;
        connection.busy_timeout(std::time::Duration::from_secs(5))?;
        connection.execute_batch(include_str!("../migrations/0001_init.sql"))?;
        let db = Self { inner: Arc::new(Mutex::new(connection)) };
        db.seed_defaults()?;
        Ok(db)
    }

    pub fn memory() -> ObResult<Self> {
        let connection = Connection::open_in_memory()?;
        connection.execute_batch(include_str!("../migrations/0001_init.sql"))?;
        let db = Self { inner: Arc::new(Mutex::new(connection)) };
        db.seed_defaults()?;
        Ok(db)
    }

    fn seed_defaults(&self) -> ObResult<()> {
        let now = Utc::now().to_rfc3339();
        let connection = self.inner.lock();
        for (key, value) in [
            ("local_only", "false"), ("autonomy_level", "1"), ("paused", "false"),
            ("privacy.microphone", "false"), ("privacy.camera", "false"), ("privacy.screen_capture", "false"),
            ("privacy.filesystem", "false"), ("privacy.browser", "false"), ("privacy.network", "true"), ("privacy.memory", "true"),
            ("approved_folders", "[]"), ("emotional_adaptation", "true")
        ] { connection.execute("INSERT OR IGNORE INTO settings(key,value_json,updated_at) VALUES(?1,?2,?3)", params![key,value,now])?; }
        for (id, kind, label, base, enabled) in [
            ("ollama", "ollama", "Ollama", "http://127.0.0.1:11434", 1),
            ("gemini", "gemini", "Google Gemini", "https://generativelanguage.googleapis.com/v1beta", 0),
            ("openai-compatible", "openai-compatible", "OpenAI Compatible", "http://127.0.0.1:1234/v1", 0)
        ] { connection.execute("INSERT OR IGNORE INTO providers(id,kind,label,base_url,enabled,config_json,updated_at) VALUES(?1,?2,?3,?4,?5,'{}',?6)", params![id,kind,label,base,enabled,now])?; }
        Ok(())
    }

    pub fn get_setting<T: serde::de::DeserializeOwned>(&self, key: &str, fallback: T) -> T {
        let result: Option<String> = self.inner.lock().query_row("SELECT value_json FROM settings WHERE key=?1", [key], |row| row.get(0)).optional().unwrap_or(None);
        result.and_then(|value| serde_json::from_str(&value).ok()).unwrap_or(fallback)
    }

    pub fn set_setting<T: serde::Serialize>(&self, key: &str, value: &T) -> ObResult<()> {
        let encoded = serde_json::to_string(value)?;
        self.inner.lock().execute("INSERT INTO settings(key,value_json,updated_at) VALUES(?1,?2,?3) ON CONFLICT(key) DO UPDATE SET value_json=excluded.value_json,updated_at=excluded.updated_at", params![key,encoded,Utc::now().to_rfc3339()])?;
        Ok(())
    }

    pub fn privacy(&self) -> PrivacyState {
        PrivacyState {
            microphone: self.get_setting("privacy.microphone", false), camera: self.get_setting("privacy.camera", false),
            screen_capture: self.get_setting("privacy.screen_capture", false), filesystem: self.get_setting("privacy.filesystem", false),
            browser: self.get_setting("privacy.browser", false), network: self.get_setting("privacy.network", true), memory: self.get_setting("privacy.memory", true),
        }
    }

    pub fn create_task(&self, title: &str, request: &str) -> ObResult<String> {
        let id = Uuid::new_v4().to_string(); let now = Utc::now().to_rfc3339();
        self.inner.lock().execute("INSERT INTO tasks(id,title,request,status,created_at,updated_at) VALUES(?1,?2,?3,'running',?4,?4)", params![id,title,request,now])?;
        self.add_task_event(&id, "request", "User request", request)?;
        Ok(id)
    }

    pub fn add_task_event(&self, task_id: &str, kind: &str, label: &str, detail: &str) -> ObResult<()> {
        let mut plan: Value = self.inner.lock().query_row("SELECT plan_json FROM tasks WHERE id=?1", [task_id], |row| row.get::<_,String>(0)).optional()?.and_then(|raw| serde_json::from_str(&raw).ok()).unwrap_or_else(|| serde_json::json!({"events":[]}));
        let event = serde_json::json!({"id":Uuid::new_v4().to_string(),"at":Utc::now(),"kind":kind,"label":label,"detail":detail});
        plan["events"].as_array_mut().ok_or_else(|| ObError::Internal("Task event store is malformed".into()))?.push(event);
        self.inner.lock().execute("UPDATE tasks SET plan_json=?2,updated_at=?3 WHERE id=?1", params![task_id,serde_json::to_string(&plan)?,Utc::now().to_rfc3339()])?;
        Ok(())
    }

    pub fn finish_task(&self, task_id: &str, status: &str, error: Option<&str>) -> ObResult<()> {
        let now = Utc::now().to_rfc3339();
        self.inner.lock().execute("UPDATE tasks SET status=?2,error=?3,updated_at=?4,completed_at=?4 WHERE id=?1", params![task_id,status,error,now])?;
        Ok(())
    }

    pub fn recent_tasks(&self, limit: usize) -> ObResult<Vec<TaskSummary>> {
        let connection = self.inner.lock();
        let mut statement = connection.prepare("SELECT id,title,status,created_at,completed_at,plan_json FROM tasks ORDER BY created_at DESC LIMIT ?1")?;
        let rows = statement.query_map([limit as i64], |row| {
            let created: String = row.get(3)?; let completed: Option<String> = row.get(4)?; let plan: String = row.get(5)?;
            let parsed: Value = serde_json::from_str(&plan).unwrap_or_else(|_| serde_json::json!({"events":[]}));
            let events: Vec<TaskEvent> = serde_json::from_value(parsed.get("events").cloned().unwrap_or_else(|| serde_json::json!([]))).unwrap_or_default();
            Ok(TaskSummary { id: row.get(0)?, title: row.get(1)?, status: row.get(2)?, created_at: DateTime::parse_from_rfc3339(&created).map(|v|v.with_timezone(&Utc)).unwrap_or_else(|_|Utc::now()), completed_at: completed.and_then(|v|DateTime::parse_from_rfc3339(&v).ok()).map(|v|v.with_timezone(&Utc)), progress: None, events })
        })?;
        Ok(rows.filter_map(Result::ok).collect())
    }

    pub fn active_task(&self) -> ObResult<Option<TaskSummary>> {
        Ok(self.recent_tasks(20)?.into_iter().find(|task| matches!(task.status.as_str(), "running" | "queued" | "waiting_approval")))
    }

    pub fn unread_notifications(&self) -> u32 {
        self.inner.lock().query_row("SELECT COUNT(*) FROM notifications WHERE read=0", [], |row| row.get::<_,i64>(0)).unwrap_or(0).max(0) as u32
    }

    pub fn execute<F, T>(&self, operation: F) -> ObResult<T> where F: FnOnce(&Connection) -> ObResult<T> { operation(&self.inner.lock()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn migrations_and_defaults_work() { let db = Database::memory().expect("database"); assert_eq!(db.get_setting::<u8>("autonomy_level",0),1); assert!(db.recent_tasks(5).expect("tasks").is_empty()); }
}
