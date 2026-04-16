use dtwm_parser::parse_source;
use serde::Serialize;
use serde_json::Value;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Serialize)]
struct CompareFile {
    declarations: Vec<CompareDecl>,
}

#[derive(Debug, Serialize)]
struct CompareDecl {
    kind: String,
    name: String,
    items: Vec<Vec<String>>,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("corpus")
}

fn collect_fixtures() -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = Vec::new();
    let dir = fixtures_dir();
    let entries = fs::read_dir(&dir).expect("corpus dir readable");
    for entry in entries {
        let entry = entry.expect("corpus entry readable");
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("dtwmrc") {
            let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if !name.starts_with("missing-") {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}

fn normalize_rust(path: &Path) -> CompareFile {
    let src = fs::read_to_string(path).expect("fixture readable");
    let rf = parse_source(&src, path).expect("parse ok");
    let declarations = rf
        .declarations
        .into_iter()
        .map(|decl| {
            let items = decl.items().into_iter().map(|item| item.tokens).collect();
            CompareDecl {
                kind: decl.kind.as_str().to_string(),
                name: decl.name,
                items,
            }
        })
        .collect();
    CompareFile { declarations }
}

fn legacy_json(cmd: &str, args: Option<&str>, path: &Path) -> Value {
    let mut command = Command::new(cmd);
    if let Some(args) = args {
        command.args(args.split_whitespace());
    }
    let output = command
        .arg(path)
        .output()
        .expect("legacy parser command failed");
    assert!(
        output.status.success(),
        "legacy parser failed for {}",
        path.display()
    );
    serde_json::from_slice(&output.stdout).expect("legacy JSON parse")
}

#[test]
fn compare_with_legacy_parser() {
    let Some(cmd) = env::var("CDE_DTWM_LEGACY_PARSER").ok() else {
        eprintln!("compare mode disabled; set CDE_DTWM_LEGACY_PARSER to enable");
        return;
    };
    let args = env::var("CDE_DTWM_LEGACY_ARGS").ok();

    for path in collect_fixtures() {
        let rust_value = serde_json::to_value(normalize_rust(&path))
            .expect("rust normalization JSON");
        let legacy_value = legacy_json(&cmd, args.as_deref(), &path);
        assert_eq!(rust_value, legacy_value, "fixture: {}", path.display());
    }
}
