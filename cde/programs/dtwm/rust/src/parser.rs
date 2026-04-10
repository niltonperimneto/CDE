//! pest-based top-level parser for dtwmrc files.

use crate::ast::{DeclKind, Declaration, ResourceFile};
use crate::error::{DtwmError, ParseError};
use pest::Parser;
use pest_derive::Parser;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[grammar = "../grammar/dtwmrc.pest"]
pub struct DtwmrcParser;

pub fn parse_source(source: &str, path: &Path) -> Result<ResourceFile, ParseError> {
    let pairs = DtwmrcParser::parse(Rule::file, source)
        .map_err(|e| ParseError::from_pest(path.to_path_buf(), source.to_string(), e))?;

    let mut file = ResourceFile::new();
    for top in pairs {
        if top.as_rule() != Rule::file {
            continue;
        }
        for decl in top.into_inner() {
            if decl.as_rule() != Rule::declaration {
                continue;
            }
            if let Some(d) = extract_decl(decl, path) {
                file.declarations.push(d);
            }
        }
    }
    Ok(file)
}

fn extract_decl(
    pair: pest::iterators::Pair<'_, Rule>,
    path: &Path,
) -> Option<Declaration> {
    let inner = pair.into_inner().next()?;
    let kind = match inner.as_rule() {
        Rule::menu_decl => DeclKind::Menu,
        Rule::keys_decl => DeclKind::Keys,
        Rule::buttons_decl => DeclKind::Buttons,
        Rule::panel_decl => DeclKind::Panel,
        Rule::box_decl => DeclKind::Box_,
        Rule::control_decl => DeclKind::Control,
        Rule::switch_decl => DeclKind::Switch,
        _ => return None,
    };

    let line = inner.as_span().start_pos().line_col().0 as u32;
    let mut parts = inner.into_inner();
    let name = parts.next()?.as_str().to_string();
    let body_pair = parts.next()?;
    let body_inner = body_pair
        .into_inner()
        .next()
        .map(|p| p.as_str().to_string())
        .unwrap_or_default();

    Some(Declaration {
        kind,
        name,
        body: body_inner,
        source: path.to_path_buf(),
        line,
    })
}

pub fn parse_file(path: &Path) -> Result<ResourceFile, DtwmError> {
    let source = std::fs::read_to_string(path).map_err(|source| DtwmError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    Ok(parse_source(&source, path)?)
}

pub fn parse_file_lossy(path: &PathBuf) -> (ResourceFile, Option<String>) {
    match parse_file(path) {
        Ok(rf) => (rf, None),
        Err(e) => (ResourceFile::new(), Some(format!("{e:?}"))),
    }
}
