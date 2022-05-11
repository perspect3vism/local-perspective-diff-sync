use std::collections::HashMap;
use petgraph::{graph::{UnGraph, DiGraph, Graph, NodeIndex}, algo::{all_simple_paths, dominators::simple_fast}};
use petgraph::dot::{Dot, Config};

pub struct Search {
    pub graph: DiGraph<usize, ()>,
    pub undirected_graph: UnGraph<usize, ()>,
    pub node_index_map: HashMap<usize, NodeIndex<u32>>
}


impl Search {
    pub fn new() -> Search {
        Search {
            graph: Graph::new(),
            undirected_graph: Graph::new_undirected(),
            node_index_map: HashMap::new()
        }
    }

    pub fn add_node(&mut self, parents: Option<Vec<NodeIndex<u32>>>, diff: usize) -> NodeIndex<u32> {
        let index = self.graph.add_node(diff.clone());
        self.undirected_graph.add_node(diff.clone());
        self.node_index_map.insert(diff, index);
        if parents.is_some() {
            for parent in parents.unwrap() {
                self.graph.add_edge(index, parent, ());
                self.undirected_graph.add_edge(index, parent, ());
            }
        }
        index
    }

    pub fn get_node_index(&self, node: &usize) -> Option<&NodeIndex<u32>> {
        self.node_index_map.get(node)
    }

    pub fn print(&self) {
        println!("Directed: {:?}\n", Dot::with_config(&self.graph, &[Config::NodeIndexLabel]));
        println!("Undirected: {:?}\n", Dot::with_config(&self.undirected_graph, &[Config::NodeIndexLabel]));
    }

    pub fn get_paths(&self, child: NodeIndex<u32>, ancestor: NodeIndex<u32>) -> Vec<Vec<NodeIndex>> {
        let paths = all_simple_paths::<Vec<_>, _>(&self.graph, child, ancestor, 0, None)
            .collect::<Vec<_>>();
        println!("Simple paths: {:#?}", paths);
        paths
    }

    pub fn find_common_ancestor(&self, root: NodeIndex<u32>, second: NodeIndex<u32>) -> Option<NodeIndex> {
        let common = simple_fast(&self.undirected_graph, root).immediate_dominator(second);
        println!("Common: {:#?}", common);
        common
    }
}

#[test]
fn ancestor_search_test() {
    let mut search = Search::new();
    let index = search.add_node(None, 0);
    let child_index = search.add_node(Some(vec![index]), 1);
    let fork_index = search.add_node(Some(vec![child_index]), 2);
    let fork_second_index = search.add_node(Some(vec![child_index]), 3);
    let fork_child = search.add_node(Some(vec![fork_index]), 4);
    let merge_entry = search.add_node(Some(vec![fork_child, fork_second_index]), 5);
    search.print();
    let is_ancestor = search.get_paths(merge_entry, child_index);
    assert_eq!(is_ancestor.len() > 0, true);

    let is_ancestor = search.get_paths(child_index, merge_entry);
    assert_eq!(is_ancestor.len() > 0, false);

    //Fork branch one; depth two
    let open_fork = search.add_node(Some(vec![merge_entry]), 6);
    let _open_fork_2 = search.add_node(Some(vec![open_fork]), 7);

    //Fork branch two; depth one
    let open_fork_3 = search.add_node(Some(vec![merge_entry]), 8);
    search.print();

    let common = search.find_common_ancestor(open_fork, open_fork_3);
    assert_eq!(common, Some(NodeIndex::new(5)));
}
