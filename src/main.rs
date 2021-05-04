mod stats;

use gfaR::Gfa;
use crate::stats::{meanNodeSize, graphSize, inputGenomes, nodeDegree, invEdges};
use std::collections::HashMap;
use std::hash::Hash;


fn formatNice(h: &HashMap<String, String>){
    for entry in h.iter(){
        print!("{}", entry.0.to_owned() + ": "+ entry.1);
    }
}


fn main() {
    println!("Hello, world!");
    let graph = gfaR::readGFA("daskldja");
    println!("{}", graph.nodes.len());
    println!("{}", graph.edges.len());
    println!("{}", meanNodeSize(&graph));
    println!("{}", graphSize(&graph));
    println!("{}", inputGenomes(&graph));
    //let u = graph.nodes;
    println!("{:?}", nodeDegree(&graph));
    println!("{}", invEdges(&graph));
    let mut a: HashMap<String, String> = HashMap::new();
    a.insert("hallo".to_owned(), graphSize(&graph).to_string());

    formatNice(&a);
}