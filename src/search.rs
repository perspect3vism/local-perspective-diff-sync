use crate::*;

//Check that a current revision is an ancesor of the networks latest_revision
pub fn check_is_ancestor(current_revision: String) -> Option<String> {
    None
}

//Gather all the diffs between revisions, assumes that the revisions are direct ancestors and not forking
pub fn gather_diffs_between(start: String, end: String) -> PerspectiveDiff {
    PerspectiveDiff {
        additions: vec![],
        removals: vec![]
    }
}

//Get diffs on a fork and return the place where fork occured relative to latest_revision() history
pub fn gather_diffs_on_fork(fork_top: String, latest_revision: String) -> (String, Vec<PerspectiveDiff>) {
    //recurse back from current_revision until parent of some entry is shared with parent of some entry on the fork
    //question: how do we get depth of agent on a given fork, how far do we go back to find some entry which could be the place where divergence occured

    (String::from("fork_start"), vec![])
}