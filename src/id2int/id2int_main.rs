use std::collections::HashMap;
use std::hash::Hash;
use clap::ArgMatches;
use gfa_reader::{Gfa, NEdge, NNode, NPath};

pub fn id2int_main(matches: &ArgMatches, graph: &Gfa){
    eprintln!("ID2INT");
    eprintln!("Convert non digit id to integer and compress id space");

    let dict: HashMap<String, u32> = HashMap::new();
    let (mut ht, nnodes) = make_hashtable(graph);
    let ne = edges_new(graph, &ht);
    let np = path_new(graph, &ht);


    // Make a meta file


}

/// Check if you destroy the order
fn make_hashtable(graph: &Gfa) -> (HashMap<&String, u32>, HashMap<u32, NNode>){
    let mut result: HashMap<&String, u32> = HashMap::new();
    let mut nodes_new: HashMap<u32, NNode> = HashMap::new();
    for (i, node) in graph.nodes.iter().enumerate(){
        result.insert(node.0, i as u32);
        nodes_new.insert(i as u32, NNode{len: node.1.len, id: i as u32, seq: node.1.seq.clone()});
    }
    (result, nodes_new)

}

/// Edges to int edges
/// Same as above
fn edges_new(graph: &Gfa, id2int: &HashMap<&String, u32>) -> Vec<NEdge>{
    let mut new_edges : Vec<NEdge>= Vec::new();
    for edge in graph.edges.iter(){
        let from = id2int.get(&edge.from).unwrap();
        let to = id2int.get(&edge.from).unwrap();
        new_edges.push(NEdge{from: *from, to: *to, from_dir: edge.from_dir, to_dir: edge.to_dir})

    }
    new_edges
}

fn path_new(graph: &Gfa, id2int: &HashMap<&String, u32>){
    let mut new_path = Vec::new();
    for path in graph.paths.iter(){
        let p: Vec<u32> = path.nodes.iter().map(|n|id2int.get(n).unwrap().clone()).collect();
        new_path.push(NPath{name: path.name.clone(), nodes: p, dir: path.dir.clone()})
    }

}