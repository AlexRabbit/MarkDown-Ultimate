//! Structured diagnostic events (JSONL) with correlation IDs.
//! On native targets, appends to `logs/hot/events.jsonl`.
//! On WASM, events are kept in a bounded in-memory ring and mirrored to `console`.

use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;
use std::collections::VecDeque;
use std::sync::{Mutex, OnceLock};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct DiagnosticEvent {
    pub schema_version: u32,
    pub event_id: String,
    pub trace_id: String,
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

pub fn emit(
    severity: Severity,
    category: &str,
    event_type: &str,
    component: &str,
    message: impl Into<String>,
    context: Option<Value>,
    error: Option<Value>,
) {
    let ev = DiagnosticEvent {
        schema_version: 1,
        event_id: Uuid::new_v4().to_string(),
        trace_id: trace_id(),
        timestamp: Utc::now(),
        severity,
        category: category.into(),
        event_type: event_type.into(),
        component: component.into(),
        message: message.into(),
        context,
        error,
    };
    let line = match serde_json::to_string(&ev) {
        Ok(l) => l,
        Err(_) => return,
    };
    persist_line(&line);
}

fn persist_line(line: &str) {
    let ring = RING.get_or_init(|| Mutex::new(VecDeque::with_capacity(RING_CAP)));
    if let Ok(mut q) = ring.lock() {
        if q.len() >= RING_CAP {
            q.pop_front();
        }
        q.push_back(line.to_string());
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::fs::{create_dir_all, OpenOptions};
        use std::io::Write;
        let _ = create_dir_all("logs/hot");
        if let Ok(mut f) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("logs/hot/events.jsonl")
        {
            let _ = writeln!(f, "{line}");
        }
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
        assert!(!recent_events(1).is_empty());
    }
}
