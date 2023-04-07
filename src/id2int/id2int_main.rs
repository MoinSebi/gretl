use std::collections::HashMap;
use std::hash::Hash;
use clap::ArgMatches;
use gfa_reader::{Gfa, NNode};

pub fn id2int_main(matches: &ArgMatches, graph: &Gfa){
    eprintln!("Running core analysis");
    eprintln!("Convert non digit id to integer and compress id space");

    let dict: HashMap<String, u32> = HashMap::new();



    // Make a meta file


}

/// Check if you destroy the order
fn make_hashtable(graph: &Gfa) -> (HashMap<&str, u32>, HashMap<u32, NNode>){
    let mut result: HashMap<&str, u32> = HashMap::new();
    let mut nodes_new: HashMap<u32, NNode> = HashMap::new();
    for (i, node) in graph.nodes.iter().enumerate(){
        result.insert(node.0, i as u32);
        nodes_new.insert(i as u32, NNode{len: node.1.len, id: i as u32, seq: node.1.seq.clone()})
    }
    (result, nodes_new)

}

fn edges_new(graph: Gfa, id2int: HashMap<&str, uy32>)