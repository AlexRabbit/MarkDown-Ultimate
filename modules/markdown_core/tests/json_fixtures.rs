//! JSON-driven GFM/CommonMark spot checks (see QTestings/fixtures/).

use mu_markdown_core::render_markdown;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct Fixture {
    name: String,
    markdown: String,
    expect_contains: Vec<String>,
    #[serde(default)]
    expect_not_contains: Vec<String>,
    #[serde(default)]
    skip: bool,
}

#[test]
fn qtestings_gfm_basic_json() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../QTestings/fixtures/gfm-basic.json");
    let raw = fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!("read {}: {e}", path.display())
    });
    let cases: Vec<Fixture> = serde_json::from_str(&raw).expect("parse fixtures json");
    assert!(!cases.is_empty(), "fixture file must not be empty");
    for case in cases {
        if case.skip {
            eprintln!("SKIP fixture {}", case.name);
            continue;
        }
        let html = render_markdown(&case.markdown);
        for needle in &case.expect_contains {
            assert!(
                html.contains(needle.as_str()),
                "fixture `{}` expected HTML to contain {:?}\nmarkdown:\n{}\nhtml:\n{}",
                case.name,
                needle,
                case.markdown,
                html
            );
        }
        for needle in &case.expect_not_contains {
            assert!(
                !html.contains(needle.as_str()),
                "fixture `{}` expected HTML to NOT contain {:?}\nhtml:\n{}",
                case.name,
                needle,
                html
            );
        }
    }
}
