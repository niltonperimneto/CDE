//! Abstract syntax tree for parsed `.dt` Action/DataType files.
//!
//! Field names mirror those defined in `include/Dt/ActionDb.h:49-130`. Every
//! record is keyed by its identifier and carries the source file it was read
//! from for round-tripping diagnostics.

use std::collections::HashMap;
use std::path::PathBuf;

/// Kind of record parsed from a `.dt` file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordKind {
    Action,
    DataAttributes,
    DataCriteria,
    Set,
}

/// One parsed record from a `.dt` file (ACTION, DATA_ATTRIBUTES, ...).
///
/// Fields are stored as a map from the uppercased keyword to its raw string
/// value. Callers that need typed access use the helper methods below or the
/// more specialised [`Action`] / [`DataType`] views.
#[derive(Debug, Clone)]
pub struct Record {
    pub kind: RecordKind,
    pub name: String,
    pub fields: HashMap<String, String>,
    pub source: PathBuf,
    pub line: u32,
}

impl Record {
    pub fn get(&self, key: &str) -> Option<&str> {
        self.fields.get(&key.to_ascii_uppercase()).map(String::as_str)
    }

    pub fn label(&self) -> Option<&str> { self.get("LABEL") }
    pub fn description(&self) -> Option<&str> { self.get("DESCRIPTION") }
    pub fn type_(&self) -> Option<&str> { self.get("TYPE") }
    pub fn exec_string(&self) -> Option<&str> { self.get("EXEC_STRING") }
    pub fn window_type(&self) -> Option<&str> { self.get("WINDOW_TYPE") }
}

/// Top-level database of all records loaded from a directory tree.
#[derive(Debug, Default)]
pub struct Database {
    pub actions: HashMap<String, Record>,
    pub data_attributes: HashMap<String, Record>,
    pub data_criteria: Vec<Record>,
    pub sets: HashMap<String, Record>,
    pub errors: Vec<String>,
}

impl Database {
    pub fn new() -> Self { Self::default() }

    pub fn insert(&mut self, record: Record) {
        match record.kind {
            RecordKind::Action => { self.actions.insert(record.name.clone(), record); }
            RecordKind::DataAttributes => { self.data_attributes.insert(record.name.clone(), record); }
            RecordKind::DataCriteria => { self.data_criteria.push(record); }
            RecordKind::Set => { self.sets.insert(record.name.clone(), record); }
        }
    }

    pub fn merge(&mut self, other: Database) {
        self.actions.extend(other.actions);
        self.data_attributes.extend(other.data_attributes);
        self.data_criteria.extend(other.data_criteria);
        self.sets.extend(other.sets);
        self.errors.extend(other.errors);
    }

    pub fn action_count(&self) -> usize { self.actions.len() }
    pub fn datatype_count(&self) -> usize { self.data_attributes.len() }
}
