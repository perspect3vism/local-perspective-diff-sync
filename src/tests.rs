#[test]
fn main_test() {
    use crate::methods::*;
    use crate::{PerspectiveDiff, LinkExpression, Link, ExpressionProof, CURRENT_REVISION};

    commit(PerspectiveDiff {
        additions: vec![LinkExpression {
            author: "did".to_string(),
            timestamp: "now".to_string(),
            data: Link::default(),
            expression_proof: ExpressionProof::default()
        }],
        removals: vec![],
    }, None);
    commit(PerspectiveDiff {
        additions: vec![LinkExpression {
            author: "did2".to_string(),
            timestamp: "now".to_string(),
            data: Link::default(),
            expression_proof: ExpressionProof::default()
        }],
        removals: vec![],
    }, None);
    let diff3 = commit(PerspectiveDiff {
        additions: vec![LinkExpression {
            author: "did3".to_string(),
            timestamp: "now".to_string(),
            data: Link::default(),
            expression_proof: ExpressionProof::default()
        }],
        removals: vec![],
    }, None);
    //Create forking entries
    let diff4 = commit(PerspectiveDiff {
        additions: vec![LinkExpression {
            author: "did4".to_string(),
            timestamp: "now".to_string(),
            data: Link::default(),
            expression_proof: ExpressionProof::default()
        }],
        removals: vec![],
    }, None);
    commit(PerspectiveDiff {
        additions: vec![LinkExpression {
            author: "did5".to_string(),
            timestamp: "now".to_string(),
            data: Link::default(),
            expression_proof: ExpressionProof::default()
        }],
        removals: vec![],
    }, Some(vec![diff3]));

    //Simulate local state as one tip of fork
    *CURRENT_REVISION.write().expect("Could not get lock on current revision") = diff4;
    render();

    //Create a merge
    let merge_result = pull();
    println!("Merge result: {:#?}", merge_result);
    render();

    //Create a new entry and simulate being out of sync and pulling
    commit(PerspectiveDiff {
        additions: vec![LinkExpression {
            author: "did6".to_string(),
            timestamp: "now".to_string(),
            data: Link::default(),
            expression_proof: ExpressionProof::default()
        }],
        removals: vec![],
    }, None);
    commit(PerspectiveDiff {
        additions: vec![LinkExpression {
            author: "did7".to_string(),
            timestamp: "now".to_string(),
            data: Link::default(),
            expression_proof: ExpressionProof::default()
        }],
        removals: vec![],
    }, None);

    *CURRENT_REVISION.write().expect("Could not get lock on current revision") = 6;
    //Fetch the latest changes
    let fetch_result = pull();
    assert_eq!(fetch_result, PerspectiveDiff {
        additions: vec![LinkExpression {
            author: "did6".to_string(),
            timestamp: "now".to_string(),
            data: Link::default(),
            expression_proof: ExpressionProof::default()
        },
        LinkExpression {
            author: "did7".to_string(),
            timestamp: "now".to_string(),
            data: Link::default(),
            expression_proof: ExpressionProof::default()
        }],
        removals: vec![],
    });
    println!("Passed first fetch");

    //Check another fetch does not return results 
    let fetch_synced = pull();
    assert_eq!(fetch_synced, PerspectiveDiff {
        additions: vec![],
        removals: vec![]
    })
}