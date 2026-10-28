//! Single-source configuration loaded from project `.env` (see `.env.example`).

use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub app_mode: String,
    pub log_level: String,
    pub github_pages_base: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app_mode: "production".into(),
            log_level: "info".into(),
            github_pages_base: "/MarkDown-Ultimate/".into(),
        }
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

pub fn load_from_env_file(path: impl AsRef<Path>) -> Result<AppConfig, ConfigError> {
    let mut cfg = AppConfig::default();
    let content = fs::read_to_string(path.as_ref()).unwrap_or_default();
    let map = parse_dotenv(&content);
    if let Some(v) = map.get("APP_MODE") {
        cfg.app_mode = v.clone();
    }
    if let Some(v) = map.get("LOG_LEVEL") {
        cfg.log_level = v.clone();
    }
    if let Some(v) = map.get("GITHUB_PAGES_BASE") {
        cfg.github_pages_base = v.clone();
    }
    Ok(cfg)
}

fn parse_dotenv(content: &str) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            out.insert(k.trim().to_string(), v.trim().trim_matches('"').to_string());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_env_keys() {
        let m = parse_dotenv("APP_MODE=dev\n# x\nLOG_LEVEL=debug");
        assert_eq!(m.get("APP_MODE").unwrap(), "dev");
        assert_eq!(m.get("LOG_LEVEL").unwrap(), "debug");
    }
}
