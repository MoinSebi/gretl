use std::collections::HashMap;
use gfa_reader::{Edge, Gfa};



/// Compute the node len
///
/// Return a vector
pub fn pos1(graph: &Gfa<()>) ->  Vec<(bool, bool)>{
    let mut result = Vec::new();

    for x in graph.edges.as_ref().unwrap().iter(){
        result.push((x.from_dir.clone(), x.to_dir.clone()));
    }
    return result
}


