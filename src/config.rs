use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Operation {
    Add {
        value: usize,
    },
    Dereference {
        times: Option<u16>,
    },
    Jmp {
        offset: Option<usize>,
        length: Option<usize>,
    },
    Offset {
        offset: usize,
    },
    RipRelative {
        offset: Option<usize>,
        length: Option<usize>,
    },
    Subtract {
        value: usize,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Signature {
    pub name: String,
    pub module: String,
    pub pattern: String,
    pub operations: Vec<Operation>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub signatures: Vec<Signature>,
}
