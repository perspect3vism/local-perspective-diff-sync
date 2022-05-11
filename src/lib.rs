#![feature(map_first_last)]
#[macro_use]
extern crate lazy_static;

use std::sync::{Mutex, RwLock};
use indexmap::IndexMap;
use serde::{Serialize, Deserialize};

pub mod search;
pub mod methods;
pub mod utils;
mod tests;

lazy_static! {
    pub static ref CHAIN: Mutex<IndexMap<usize, PerspectiveDiffEntry>> = Mutex::new(IndexMap::new());
    pub static ref CURRENT_REVISION: RwLock<usize> = RwLock::new(0);
    pub static ref INC: RwLock<usize> =  RwLock::new(0);
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Hash, Clone, Default)]
pub struct Link {
    pub source: Option<String>,
    pub predicate: Option<String>,
    pub target: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Hash, Clone, Default)]
pub struct ExpressionProof {
    pub signature: String,
    pub key: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Hash, Clone)]
pub struct LinkExpression {
    pub author: String,
    pub timestamp: String,
    pub data: Link,
    pub expression_proof: ExpressionProof 
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Hash, Clone)]
pub struct PerspectiveDiff {
    pub additions: Vec<LinkExpression>,
    pub removals: Vec<LinkExpression>
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Hash, Clone)]
pub struct PerspectiveDiffEntry {
    pub parents: Vec<usize>,
    pub diff: PerspectiveDiff,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct MergeEntry {
    pub parents: Vec<String>
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Snapshot {
    pub links: Vec<LinkExpression>,
    pub last_snapshot: String,
}