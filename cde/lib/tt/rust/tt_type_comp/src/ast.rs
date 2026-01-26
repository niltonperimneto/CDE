use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Ptype {
    pub name: String,
    pub start_string: Option<String>,
    pub signatures: Vec<Signature>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Signature {
    pub scope: Scope,
    pub op: String,
    pub args: Vec<Arg>,
    pub action: Action,
    pub opnum: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Scope {
    Session,
    File,
    Both,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Arg {
    pub mode: ArgMode,
    pub vtype: String,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum ArgMode {
    In,
    Out,
    InOut,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Action {
    Start,
    Queue,
    Observe,
}
