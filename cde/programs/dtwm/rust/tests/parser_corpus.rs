use dtwm_parser::{parse_source, DeclKind};
use std::{fs, path::Path};

fn fixture(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("corpus")
        .join(name);
    fs::read_to_string(path).expect("fixture readable")
}

#[test]
fn parses_basic_dtwmrc_fixture() {
    let src = fixture("basic.dtwmrc");
    let rf = parse_source(&src, Path::new("basic.dtwmrc")).expect("parse ok");

    assert_eq!(rf.declarations.len(), 3);
    assert_eq!(rf.declarations[0].kind, DeclKind::Menu);
    assert_eq!(rf.declarations[0].name, "DtRootMenu");
    assert_eq!(rf.declarations[0].source, Path::new("basic.dtwmrc"));
    assert!(rf.declarations[0].line > 0);
    assert_eq!(rf.declarations[1].kind, DeclKind::Keys);
    assert_eq!(rf.declarations[2].kind, DeclKind::Buttons);
}

#[test]
fn rejects_invalid_dtwmrc_fixture() {
    let src = fixture("missing-brace.dtwmrc");
    let err = parse_source(&src, Path::new("missing-brace.dtwmrc")).unwrap_err();
    assert!(!err.message.is_empty());
    assert!(err.file.ends_with("missing-brace.dtwmrc"));
}

#[test]
fn parses_nested_menu_fixture() {
    let src = fixture("nested-menus.dtwmrc");
    let rf = parse_source(&src, Path::new("nested-menus.dtwmrc")).expect("parse ok");

    assert!(rf.declarations.len() >= 4);
    let root = rf.find(DeclKind::Menu, "DtRootMenu").expect("root menu");
    assert_eq!(root.source, Path::new("nested-menus.dtwmrc"));
    assert!(root.body.contains("FileMenu"));
    assert!(root.items().iter().any(|item| item.tokens.iter().any(|t| t == "f.menu")));
}
