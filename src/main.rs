#![feature(map_first_last)]
use sha2::{Sha256, Digest};
use base64ct::{Base64, Encoding};

use perspective_diff_sync::*;

pub fn latest_revision() -> String {
    match CHAIN.lock().expect("Could not get lock on chain").last_entry() {
        Some(diff_hash) => diff_hash.key().to_string(),
        None => String::from("root")
    }
}

pub fn current_revision() -> String {
    match CHAIN.lock().expect("Could not get lock on chain").last_entry() {
        Some(diff_hash) => diff_hash.key().to_string(),
        None => String::from("root")
    }
}

pub fn pull() -> PerspectiveDiff {
    if latest_revision() != current_revision() {
        let is_ancestor = search::check_is_ancestor(current_revision());
        if is_ancestor.is_some() {
            //Get all diffs between is_ancestor result and current_revision
            search::gather_diffs_between(current_revision(), is_ancestor.unwrap())
        } else {
            //There is a fork, find all the missing diffs from a fork and apply in merge with latest_revision() as parent
            search::gather_diffs_on_fork(current_revision(), latest_revision())
        }
    } else {
        PerspectiveDiff {
            additions: vec![],
            removals: vec![]
        }
    }
}

pub fn render() {

}

pub fn commit(diff: PerspectiveDiff, inject_parent: Option<String>) {
    let diffs_before_snapshot = 10;
    //Hash diff commit
    let encoded_diff: Vec<u8> = bincode::serialize(&diff).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(encoded_diff);
    let hash = hasher.finalize();
    let base64_hash = Base64::encode_string(&hash);

    //Get last parent
    let parent = if inject_parent.is_none() {
        latest_revision()
    } else {
        inject_parent.unwrap()
    };

    let entry = PerspectiveDiffEntry {
        parent: parent,
        diff: diff
    };
    CHAIN.lock().expect("Could not get lock on chain").insert(base64_hash, entry);
}

fn main() {
    println!("Hello, world!");
}
