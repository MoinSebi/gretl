use std::collections::HashMap;
use gfa_reader::{Gfa, NEdge, NNode, NPath};

/// Create id2int hashmap and new nodes (in u32 space)
pub fn id2int_nnodes(graph: &Gfa) -> (HashMap<&String, u32>, HashMap<u32, NNode>){
    let mut result: HashMap<&String, u32> = HashMap::new();
    let mut nodes_new: HashMap<u32, NNode> = HashMap::new();
    for (index, (node_id, node_info)) in graph.nodes.iter().enumerate(){
        result.insert(node_id, index as u32);
        nodes_new.insert(index as u32, NNode{len: node_info.len, id: index as u32, seq: node_info.seq.clone()});
    }
    (result, nodes_new)

}

/// Edges to numeric edges
///
/// Usage the previously constructed helper-hashmap
pub fn nedges(graph: &Gfa, id2int: &HashMap<&String, u32>) -> Vec<NEdge>{
    let mut new_edges : Vec<NEdge>= Vec::new();
    for edge in graph.edges.iter(){
        let from = id2int.get(&edge.from).unwrap();
        let to = id2int.get(&edge.from).unwrap();
        new_edges.push(NEdge{from: *from, to: *to, from_dir: edge.from_dir, to_dir: edge.to_dir})
    }
    new_edges
}


/// Path to numeric paths
///
/// Usage the previously constructed helper-hashmap
pub fn path_new(graph: &Gfa, id2int: &HashMap<&String, u32>) -> (Vec<NPath>, HashMap<String, usize>){
    let mut new_path = Vec::new();
    let mut path2id = HashMap::new();
    for (index, path) in graph.paths.iter().enumerate(){
        let p: Vec<u32> = path.nodes.iter().map(|n|id2int.get(n).unwrap().clone()).collect();
        new_path.push(NPath{name: path.name.clone(), nodes: p, dir: path.dir.clone()});
        path2id.insert(path.name.clone(), index);
    }
    return (new_path, path2id)
}