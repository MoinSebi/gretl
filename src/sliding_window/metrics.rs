use std::collections::{HashMap, HashSet};
use gfa_reader::Gfa;
use crate::node_list::wrapper::make_mapper;

/// Counting the amount
pub fn calculate_core12(graph: &Gfa) -> HashMap<u32, u32>{

    // Initialization hashmap
    let mut count: HashMap<u32,u32> = HashMap::new();
    for x in &graph.nodes{
        count.insert(x.0.parse().unwrap(),  0);
    }

    // Calculate the amount of sequence and the amount of nodes
    for x in &graph.paths{
        // Count every node only once
        let v: HashSet<_> = x.nodes.iter().cloned().collect();
        for y in v{
            *count.get_mut(&y.parse().unwrap()).unwrap() += 1;
        }
    }
    count.shrink_to_fit();
    count
}

/// Compute the node len
///
pub fn node_len(graph: &Gfa) ->  HashMap<u32, u32> {
    let mapper = make_mapper(graph);

    let mut result = HashMap::new();
    for (id, node) in graph.nodes.iter() {
        let index = mapper.get(id).unwrap();
        result.insert(index.clone() as u32, node.len as u32);
    }
    return result
}


