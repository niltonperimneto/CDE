use dt_parser::{parse_source, RecordKind};
use std::{fs, path::Path};

fn fixture(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("corpus")
        .join(name);
    fs::read_to_string(path).expect("fixture readable")
}

#[test]
fn parses_basic_dtfile_fixture() {
    let src = fixture("basic.dt");
    let records = parse_source(&src, Path::new("basic.dt")).expect("parse ok");

    assert_eq!(records.len(), 2);
    assert_eq!(records[0].kind, RecordKind::Action);
    assert_eq!(records[0].name, "Edit");
    assert_eq!(records[1].kind, RecordKind::DataAttributes);
    assert_eq!(records[1].name, "TEXTFILE");
}

#[test]
fn rejects_invalid_dtfile_fixture() {
    let src = fixture("missing-name.dt");
    let err = parse_source(&src, Path::new("missing-name.dt")).unwrap_err();
    assert!(!err.message.is_empty());
}

#[test]
fn parses_comments_and_quotes_fixture() {
    let src = fixture("comments-and-quotes.dt");
    let records = parse_source(&src, Path::new("comments-and-quotes.dt")).expect("parse ok");

    assert_eq!(records.len(), 1);
    assert_eq!(records[0].kind, RecordKind::Action);
    assert_eq!(records[0].name, "OpenFile");
    assert_eq!(records[0].source, Path::new("comments-and-quotes.dt"));
    assert!(records[0].line > 0);
    assert_eq!(records[0].label(), Some("Open \"File\""));
    assert_eq!(
        records[0].exec_string(),
        Some("/usr/bin/xdg-open \"%Arg_1%\"")
    );
    assert_eq!(records[0].description(), Some("Uses the default viewer."));
}
