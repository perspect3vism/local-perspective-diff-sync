use sha2::{Sha256, Digest};
use base64ct::{Base64, Encoding};
use petgraph::graph::NodeIndex;

use crate::{CHAIN, search::Search, PerspectiveDiff};

pub fn generate_diff_hash(diff: &PerspectiveDiff) -> String {
    let encoded_diff: Vec<u8> = bincode::serialize(diff).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(encoded_diff);
    let hash = hasher.finalize();
    Base64::encode_string(&hash)
}

pub fn populate_search() -> Search {
    let mut search = Search::new();
    //Populate search graph with all items from local CHAIN; will be replaced with chunked holochain DHT calls
    //Where we look up the chain at chunk size of N and keep making search operations on received state at each iteration
    for diff in CHAIN.lock().expect("Could not get lock").iter() {
        let persp_diff = diff.1;
        if persp_diff.parents.len() > 0 {
            let parents = persp_diff.parents.clone().into_iter()
                .map(|hash| search.get_node_index(&hash).unwrap().clone())
                .collect::<Vec<NodeIndex>>();
            search.add_node(Some(parents), diff.0.clone());
        } else {
            search.add_node(None, diff.0.clone());
        }
    }
    search
}
