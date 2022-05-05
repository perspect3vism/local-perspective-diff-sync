use petgraph::graph::NodeIndex;
use sha2::{Sha256, Digest};
use base64ct::{Base64, Encoding};

use crate::{CHAIN, search, PerspectiveDiff, PerspectiveDiffEntry};

//Represents the latest revision as seen by the DHT
pub fn latest_revision() -> String {
    match CHAIN.lock().expect("Could not get lock on chain").last_entry() {
        Some(diff_hash) => diff_hash.key().to_string(),
        None => String::from("root")
    }
}

//Represents the current revision as seen by our local state
pub fn current_revision() -> String {
    match CHAIN.lock().expect("Could not get lock on chain").last_entry() {
        Some(diff_hash) => diff_hash.key().to_string(),
        None => String::from("root")
    }
}

pub fn pull() -> PerspectiveDiff {
    let latest = latest_revision();
    let current = current_revision();
    //Latest DHT state is not equal to users local state; we are not sync'd
    if latest != current {
        //Start search
        let mut search = search::Search::new();
        //Populate search graph with all items from local CHAIN; will be replaced with chunked holochain DHT calls
        //Where we look up the chain at chunk size of N and keep making search operations on received state at each iteration
        for diff in CHAIN.lock().expect("Could not get lock").iter() {
            let persp_diff = diff.1;
            if persp_diff.parents.len() > 0 {
                let parents = persp_diff.parents.clone().into_iter()
                    .map(|hash| search.get_node_index(hash).unwrap().clone())
                    .collect::<Vec<NodeIndex>>();
                search.add_node(Some(parents), diff.0.clone());
            } else {
                search.add_node(None, diff.0.clone());
            }
        }
        //Get index for current and latest indexes
        let current_index = search.get_node_index(current).expect("Could not find value in map").clone();
        let latest_index = search.get_node_index(latest).expect("Could not find value in map").clone();

        //Check if latest diff is a child of current diff
        let ancestor_status = search.get_paths(latest_index.clone(), current_index.clone());

        if ancestor_status.len() > 0 {
            //Latest diff contains in its chain our current diff, fast forward and get all changes between now and then
            
            //Get all diffs between is_ancestor latest and current_revision
            //ancestor status contains all paths between latest and current revision, this can be used to get all the diffs when all paths are dedup'd together
            //Then update current revision to latest revision

            PerspectiveDiff {
                additions: vec![],
                removals: vec![]
            }
        } else {
            //There is a fork, find all the diffs from a fork and apply in merge with latest and current revisions as parents
            //Calculate the place where a common ancestor is shared between current and latest revisions
            //Common ancestor is then used as the starting point of gathering diffs on a fork

            //let common_ancestor = search.find_common_ancestor(current_index, latest_index).expect("Could not find common ancestor");
            //let paths = search.get_paths(latest_index.clone(), common_ancestor.clone());
            //Use items in path to recurse from common_ancestor going in direction of fork
            
            //Create the merge entry
            //search::add_node();
            //CHAIN.insert()

            PerspectiveDiff {
                additions: vec![],
                removals: vec![]
            }
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

pub fn commit(diff: PerspectiveDiff, inject_parent: Option<Vec<String>>) {
    let diffs_before_snapshot = 10;
    //Hash diff commit
    let encoded_diff: Vec<u8> = bincode::serialize(&diff).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(encoded_diff);
    let hash = hasher.finalize();
    let base64_hash = Base64::encode_string(&hash);

    //Get last parent
    let parent = if inject_parent.is_none() {
        vec![latest_revision()]
    } else {
        inject_parent.unwrap()
    };

    let entry = PerspectiveDiffEntry {
        parents: parent,
        diff: diff
    };
    CHAIN.lock().expect("Could not get lock on chain").insert(base64_hash, entry);
}

fn main() {
    println!("Hello world")
}