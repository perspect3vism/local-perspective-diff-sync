#![feature(map_first_last)]
#[macro_use]
extern crate lazy_static;

use std::collections::BTreeMap;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};

pub mod search;
pub mod methods;

lazy_static! {
    pub static ref CHAIN: Mutex<BTreeMap<String, PerspectiveDiffEntry>> = Mutex::new(BTreeMap::new());
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Hash, Clone)]
pub struct Link {
    pub source: Option<String>,
    pub predicate: Option<String>,
    pub target: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Hash, Clone)]
pub struct ExpressionProof {
    pub signature: String,
    pub key: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Hash, Clone)]
pub struct LinkExpression {
    pub author: String,
    pub timestamp: String,
    pub data: Vec<Link>,
    pub expression_proof: ExpressionProof 
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Hash, Clone)]
pub struct PerspectiveDiff {
    pub additions: Vec<LinkExpression>,
    pub removals: Vec<LinkExpression>
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Hash)]
pub struct PerspectiveDiffEntry {
    pub parents: Vec<String>,
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