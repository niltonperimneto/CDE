//! pest-based parser for `.dt` files.
//!
//! The heavy lifting is in `grammar/dtfile.pest`. This module walks the pest
//! parse tree and produces [`Record`] values for the caller. No unsafe code.

use crate::ast::{Record, RecordKind};
use crate::error::ParseError;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[grammar = "../grammar/dtfile.pest"]
pub struct DtFileParser;

/// Parse a single `.dt` source string into a list of [`Record`]s.
pub fn parse_source(source: &str, path: &Path) -> Result<Vec<Record>, ParseError> {
    let pairs = DtFileParser::parse(Rule::file, source)
        .map_err(|e| ParseError::from_pest(path.to_path_buf(), source.to_string(), e))?;

    let mut out = Vec::new();
    for file_pair in pairs {
        if file_pair.as_rule() != Rule::file {
            continue;
        }
        for rec in file_pair.into_inner() {
            if rec.as_rule() != Rule::record {
                continue;
            }
            if let Some(record) = extract_record(rec, path) {
                out.push(record);
            }
        }
    }
    Ok(out)
}

fn extract_record(
    pair: pest::iterators::Pair<'_, Rule>,
    path: &Path,
) -> Option<Record> {
    let inner = pair.into_inner().next()?;
    let kind = match inner.as_rule() {
        Rule::action_record => RecordKind::Action,
        Rule::data_attributes => RecordKind::DataAttributes,
        Rule::data_criteria => RecordKind::DataCriteria,
        Rule::set_record => RecordKind::Set,
        _ => return None,
    };

    let line = inner.as_span().start_pos().line_col().0 as u32;
    let mut parts = inner.into_inner();
    let name = parts.next()?.as_str().to_string();
    let block = parts.next()?;

    let mut fields: HashMap<String, String> = HashMap::new();
    for item in block.into_inner() {
        if item.as_rule() != Rule::field {
            continue;
        }
        let mut fp = item.into_inner();
        let key = fp.next()?.as_str().to_ascii_uppercase();
        let value = fp.next()?.as_str().trim().to_string();
        // Strip surrounding quotes if the entire value is quoted.
        let value = unquote(&value);
        fields.insert(key, value);
    }

    Some(Record {
        kind,
        name,
        fields,
        source: path.to_path_buf(),
        line,
    })
}

fn unquote(s: &str) -> String {
    let trimmed = s.trim();
    if trimmed.len() >= 2 && trimmed.starts_with('"') && trimmed.ends_with('"') {
        let inner = &trimmed[1..trimmed.len() - 1];
        inner.replace("\\\"", "\"").replace("\\\\", "\\")
    } else {
        trimmed.to_string()
    }
}

/// Parse a `.dt` file from disk.
pub fn parse_file(path: &Path) -> Result<Vec<Record>, crate::error::DtError> {
    let source = std::fs::read_to_string(path).map_err(|source| crate::error::DtError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    Ok(parse_source(&source, path)?)
}

/// Convenience wrapper for FFI: ignore I/O errors and log parse failures.
pub fn parse_file_lossy(path: &PathBuf) -> (Vec<Record>, Option<String>) {
    match parse_file(path) {
        Ok(records) => (records, None),
        Err(e) => (Vec::new(), Some(format!("{e:?}"))),
    }
}
