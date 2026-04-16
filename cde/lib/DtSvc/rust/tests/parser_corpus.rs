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
