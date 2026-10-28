//! Best-effort PII/token redaction before events leave the process.

use regex::Regex;
use serde_json::Value;
use std::sync::OnceLock;

fn patterns() -> &'static [Regex] {
    static CACHE: OnceLock<Vec<Regex>> = OnceLock::new();
    CACHE.get_or_init(|| {
        [
            r"(?i)[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}",
            r"ghp_[A-Za-z0-9]{20,}",
            r"github_pat_[A-Za-z0-9_]{20,}",
            r"sk-[A-Za-z0-9]{20,}",
            r"AKIA[0-9A-Z]{16}",
            r"(?i)\b(?:api[_-]?key|token|password|secret)\s*[:=]\s*\S+",
            r"-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----",
        ]
        .into_iter()
        .filter_map(|p| Regex::new(p).ok())
        .collect()
    })
}

pub fn redact_text(input: &str) -> String {
    let mut out = input.to_string();
    for re in patterns() {
        out = re.replace_all(&out, "[REDACTED]").into_owned();
    }
    out
}

pub fn redact_value(value: &Value) -> Value {
    match value {
        Value::String(s) => Value::String(redact_text(s)),
        Value::Array(items) => Value::Array(items.iter().map(redact_value).collect()),
        Value::Object(map) => Value::Object(
            map.iter()
                .map(|(k, v)| (k.clone(), redact_value(v)))
                .collect(),
        ),
        other => other.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redacts_email_and_token() {
        let s = redact_text("user@example.com leaked ghp_1234567890123456789012345678901234");
        assert!(!s.contains("example.com"));
        assert!(s.contains("[REDACTED]"));
    }
}
