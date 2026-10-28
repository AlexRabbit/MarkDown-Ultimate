//! Structured diagnostic events (JSONL) with correlation IDs.

mod redact;

use chrono::{DateTime, Utc};
use redact::{redact_text, redact_value};
use serde::Serialize;
use serde_json::Value;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct DiagnosticEvent {
    pub schema_version: u32,
    pub event_id: String,
    pub trace_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_event_id: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub severity: Severity,
    pub category: String,
    pub event_type: String,
    pub component: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Value>,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Debug,
    Info,
    Warn,
    Error,
    Critical,
}

static TRACE: OnceLock<Mutex<String>> = OnceLock::new();
static RING: OnceLock<Mutex<VecDeque<String>>> = OnceLock::new();
static DROPPED: AtomicU64 = AtomicU64::new(0);
const RING_CAP: usize = 512;

fn trace_id() -> String {
    TRACE
        .get_or_init(|| Mutex::new(Uuid::new_v4().to_string()))
        .lock()
        .map(|g| g.clone())
        .unwrap_or_else(|_| Uuid::new_v4().to_string())
}

pub fn new_trace() -> String {
    let id = Uuid::new_v4().to_string();
    if let Some(lock) = TRACE.get() {
        if let Ok(mut g) = lock.lock() {
            *g = id.clone();
        }
    }
    id
}

pub fn dropped_events() -> u64 {
    DROPPED.load(Ordering::Relaxed)
}

pub fn emit(
    severity: Severity,
    category: &str,
    event_type: &str,
    component: &str,
    message: impl Into<String>,
    context: Option<Value>,
    error: Option<Value>,
) {
    emit_with_parent(
        severity,
        category,
        event_type,
        component,
        message,
        context,
        error,
        None,
        None,
    );
}

pub fn emit_with_parent(
    severity: Severity,
    category: &str,
    event_type: &str,
    component: &str,
    message: impl Into<String>,
    context: Option<Value>,
    error: Option<Value>,
    operation_id: Option<String>,
    parent_event_id: Option<String>,
) {
    let message = redact_text(&message.into());
    let context = context.map(|v| redact_value(&v));
    let error = error.map(|v| redact_value(&v));
    let ev = DiagnosticEvent {
        schema_version: 2,
        event_id: Uuid::new_v4().to_string(),
        trace_id: trace_id(),
        operation_id,
        parent_event_id,
        timestamp: Utc::now(),
        severity,
        category: category.into(),
        event_type: event_type.into(),
        component: component.into(),
        message,
        context,
        error,
    };
    let line = match serde_json::to_string(&ev) {
        Ok(l) => l,
        Err(_) => {
            DROPPED.fetch_add(1, Ordering::Relaxed);
            return;
        }
    };
    persist_line(&line);
}

fn logs_hot_path() -> String {
    std::env::var("LOGS_DIR")
        .map(|d| format!("{d}/hot/events.jsonl"))
        .unwrap_or_else(|_| "logs/hot/events.jsonl".into())
}

fn persist_line(line: &str) {
    let ring = RING.get_or_init(|| Mutex::new(VecDeque::with_capacity(RING_CAP)));
    if let Ok(mut q) = ring.lock() {
        if q.len() >= RING_CAP {
            q.pop_front();
            DROPPED.fetch_add(1, Ordering::Relaxed);
        }
        q.push_back(line.to_string());
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::fs::{create_dir_all, OpenOptions};
        use std::io::Write;
        let path = logs_hot_path();
        if let Some(parent) = std::path::Path::new(&path).parent() {
            let _ = create_dir_all(parent);
        }
        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(&path) {
            let _ = writeln!(f, "{line}");
        }
        let meta_path = std::env::var("LOGS_DIR")
            .map(|d| format!("{d}/meta/logger-health.json"))
            .unwrap_or_else(|_| "logs/meta/logger-health.json".into());
        if let Some(parent) = std::path::Path::new(&meta_path).parent() {
            let _ = create_dir_all(parent);
        }
        let health = serde_json::json!({
            "schema_version": 2,
            "dropped_events": dropped_events(),
            "ring_capacity": RING_CAP,
            "pii_redaction": "best_effort",
        });
        let _ = std::fs::write(meta_path, health.to_string());
    }
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::console::log_1(&line.into());
    }
}

pub fn recent_events(limit: usize) -> Vec<String> {
    RING
        .get()
        .and_then(|m| m.lock().ok())
        .map(|q| q.iter().rev().take(limit).cloned().collect())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emits_json_shape() {
        emit(
            Severity::Info,
            "test",
            "unit",
            "mu_diagnostics",
            "hello",
            None,
            None,
        );
        assert!(!recent_events(5).is_empty());
    }
}
