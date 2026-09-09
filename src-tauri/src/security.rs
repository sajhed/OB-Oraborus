use crate::{audit::AuditLog, db::Database, error::{ObError, ObResult}, models::{ApprovalRequest, RiskLevel}};
use chrono::{Duration, Utc};
use regex::Regex;
use rusqlite::{params, OptionalExtension};
use serde_json::Value;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, Clone)] pub enum GateDecision { Allow, Approval(ApprovalRequest), Deny(String) }
#[derive(Clone)] pub struct SecurityGate { db: Database, audit: AuditLog }
impl SecurityGate {
    pub fn new(db: Database, audit: AuditLog) -> Self { Self { db, audit } }
    pub fn evaluate(&self, task_id: &str, action: &str, risk: RiskLevel, summary: &str, payload: Value) -> ObResult<GateDecision> {
        let paused: bool = self.db.get_setting("paused", false);
        if paused { return Ok(GateDecision::Deny("OB is paused".into())); }
        let level: u8 = self.db.get_setting("autonomy_level", 1);
        let needs_approval = matches!(risk, RiskLevel::Destructive | RiskLevel::Critical) || matches!(risk, RiskLevel::Sensitive) && level < 2;
        if needs_approval {
            let approval = ApprovalRequest { id: Uuid::new_v4().to_string(), action: action.into(), risk: risk.clone(), summary: summary.into(), expires_at: Utc::now() + Duration::minutes(5) };
            self.db.execute(|connection| { connection.execute("INSERT INTO approvals(id,task_id,action,risk,summary,payload_json,status,created_at,expires_at) VALUES(?1,?2,?3,?4,?5,?6,'pending',?7,?8)",params![approval.id,task_id,action,format!("{risk:?}").to_uppercase(),summary,serde_json::to_string(&payload)?,Utc::now().to_rfc3339(),approval.expires_at.to_rfc3339()])?; Ok(()) })?;
            self.audit.record("security",action,"user",&risk,"waiting_approval",summary,serde_json::json!({"approvalId":approval.id}))?;
            Ok(GateDecision::Approval(approval))
        } else { self.audit.record("security",action,"user",&risk,"allowed",summary,serde_json::json!({}))?; Ok(GateDecision::Allow) }
    }

    pub fn resolve(&self, approval_id: &str, approved: bool) -> ObResult<(String,String,Value)> {
        self.db.execute(|connection| {
            let record: Option<(String,String,String,String)> = connection.query_row("SELECT task_id,action,payload_json,expires_at FROM approvals WHERE id=?1 AND status='pending'",[approval_id],|row|Ok((row.get(0)?,row.get(1)?,row.get(2)?,row.get(3)?))).optional()?;
            let (task,action,payload,expires) = record.ok_or_else(||ObError::Validation("Approval is missing or already resolved".into()))?;
            let expiration = chrono::DateTime::parse_from_rfc3339(&expires).map_err(|_|ObError::Validation("Approval expiry is invalid".into()))?.with_timezone(&Utc);
            if expiration < Utc::now() { connection.execute("UPDATE approvals SET status='expired',resolved_at=?2 WHERE id=?1",params![approval_id,Utc::now().to_rfc3339()])?; return Err(ObError::PermissionRequired("Approval expired".into())); }
            connection.execute("UPDATE approvals SET status=?2,resolved_at=?3 WHERE id=?1",params![approval_id,if approved{"approved"}else{"denied"},Utc::now().to_rfc3339()])?;
            if !approved { return Err(ObError::PermissionRequired("User denied the action".into())); }
            Ok((task,action,serde_json::from_str(&payload)?))
        })
    }

    pub fn validate_terminal(command: &str) -> ObResult<RiskLevel> {
        if command.len() > 8_000 || command.contains('\0') { return Err(ObError::Validation("Command input is invalid".into())); }
        let critical = Regex::new(r"(?i)(format\s+[a-z]:|diskpart|bcdedit|cipher\s+/w|remove-item\s+.+-recurse|del\s+/[sq]|shutdown\s+/[srt]|reg\s+(delete|add)|set-mppreference|net\s+user|invoke-expression|downloadstring)").expect("terminal risk pattern");
        let destructive = Regex::new(r"(?i)(remove-item|rmdir|\bdel\b|stop-process|taskkill|git\s+reset\s+--hard|git\s+clean\s+-[a-z]*f)").expect("terminal destructive pattern");
        if critical.is_match(command) { Ok(RiskLevel::Critical) } else if destructive.is_match(command) { Ok(RiskLevel::Destructive) } else { Ok(RiskLevel::Sensitive) }
    }

    pub fn approved_path(&self, candidate: &Path, allow_new_leaf: bool) -> ObResult<PathBuf> {
        let privacy = self.db.privacy(); if !privacy.filesystem { return Err(ObError::PermissionRequired("Filesystem access is disabled in Privacy Center".into())); }
        let roots: Vec<PathBuf> = self.db.get_setting("approved_folders", Vec::<PathBuf>::new());
        if roots.is_empty() { return Err(ObError::RequiresConfiguration("Choose at least one approved folder in Privacy Center".into())); }
        let canonical = if candidate.exists() { candidate.canonicalize()? } else if allow_new_leaf { let parent=candidate.parent().ok_or_else(||ObError::Validation("Path has no parent".into()))?.canonicalize()?; parent.join(candidate.file_name().ok_or_else(||ObError::Validation("Path has no filename".into()))?) } else { return Err(ObError::Validation("Path does not exist".into())); };
        let allowed = roots.iter().filter_map(|root|root.canonicalize().ok()).any(|root|canonical.starts_with(root));
        if !allowed { return Err(ObError::SecurityBlocked("Path is outside approved folders".into())); }
        Ok(canonical)
    }
}

#[cfg(test)] mod tests { use super::*; #[test] fn dangerous_commands_are_critical() { assert_eq!(SecurityGate::validate_terminal("format C:").expect("risk"),RiskLevel::Critical); assert_eq!(SecurityGate::validate_terminal("Get-ChildItem").expect("risk"),RiskLevel::Sensitive); } }
