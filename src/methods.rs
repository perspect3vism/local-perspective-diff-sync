use std::ops::Index;

use petgraph::graph::NodeIndex;

use crate::{CHAIN, PerspectiveDiff, PerspectiveDiffEntry, utils::{populate_search, generate_diff_hash}};

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
        let search = populate_search();
        //Get index for current and latest indexes
        let current_index = search.get_node_index(&current).expect("Could not find value in map").clone();
        let latest_index = search.get_node_index(&latest).expect("Could not find value in map").clone();

        //Check if latest diff is a child of current diff
        let ancestor_status = search.get_paths(latest_index.clone(), current_index.clone());

        if ancestor_status.len() > 0 {
            //Latest diff contains in its chain our current diff, fast forward and get all changes between now and then
            
            //Get all diffs between is_ancestor latest and current_revision
            //ancestor status contains all paths between latest and current revision, this can be used to get all the diffs when all paths are dedup'd together
            //Then update current revision to latest revision
            let mut diffs: Vec<NodeIndex> = ancestor_status.into_iter().flatten().collect();
            diffs.dedup();
            let mut out = PerspectiveDiff {
                additions: vec![],
                removals: vec![]
            };
            let mut chain = CHAIN.lock().expect("Could not get lock");

            for diff in diffs {
                //Remove from chain so we can get ownership
                let current_diff = chain.remove(
                    search.graph.index(diff)
                );
                if let Some(val) = current_diff {
                    out.additions.append(&mut val.diff.additions.clone());
                    out.removals.append(&mut val.diff.removals.clone());
                    //Add value back to chain
                    chain.insert(search.graph.index(diff).clone(), val);
                }
            }
            out
        } else {
            //There is a fork, find all the diffs from a fork and apply in merge with latest and current revisions as parents
            //Calculate the place where a common ancestor is shared between current and latest revisions
            //Common ancestor is then used as the starting point of gathering diffs on a fork

            let search = populate_search();
            let common_ancestor = search.find_common_ancestor(current_index, latest_index).expect("Could not find common ancestor");
            let paths = search.get_paths(latest_index.clone(), common_ancestor.clone());
            let mut fork_direction: Option<Vec<NodeIndex>> = None;

            //Use items in path to recurse from common_ancestor going in direction of fork
            for path in paths {
                if path.contains(&current_index) {
                    fork_direction = Some(path);
                    break
                };
            }
            let mut merge_entry = PerspectiveDiff {
                additions: vec![],
                removals: vec![]
            };
            let mut chain = CHAIN.lock().expect("Could not get lock");

            if let Some(diffs) = fork_direction {    
                for diff in diffs {
                    //Remove from chain so we can get ownership
                    let current_diff = chain.remove(
                        search.graph.index(diff)
                    );
                    if let Some(val) = current_diff {
                        merge_entry.additions.append(&mut val.diff.additions.clone());
                        merge_entry.removals.append(&mut val.diff.removals.clone());
                        //Add value back to chain
                        chain.insert(search.graph.index(diff).clone(), val);
                    }
                }
            }

            let hash = generate_diff_hash(&merge_entry);
            
            //Create the merge entry
            chain.insert(hash, PerspectiveDiffEntry {
                parents: vec![latest, current],
                diff: merge_entry.clone()
            });

            merge_entry
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
    //let diffs_before_snapshot = 10;
    //Hash diff commit
    let base64_hash = generate_diff_hash(&diff);

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