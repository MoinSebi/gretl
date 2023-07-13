use std::collections::HashMap;
use gfa_reader::{Edge, Gfa};

pub fn values(graph: &Gfa) -> &Vec<Edge>{
    let mut a = &graph.edges;
    a.sort_by(|a, b| a.from.parse::<u32>().unwrap().cmp(&b.from.parse::<u32>().unwrap()));
}


/// Compute the node len
///
/// Return a vector
pub fn pos1(graph: &Gfa) ->  HashMap<&Edge, bool>{
    let mut result = HashMap::new();

    for x in graph.edges.iter(){
        result.insert(x, x.from_dir);
    }
    return result
}


/// Compute the node len
///
/// Return a vector
pub fn pos2(graph: &Gfa) ->  HashMap<&Edge, bool>{
    let mut result = HashMap::new();

    for x in graph.edges.iter(){
        result.insert(x, x.to_dir);
    }
    return result
}