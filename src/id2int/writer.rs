use std::ascii::escape_default;
use std::cmp::min;
use gfa_reader::NGfa;
use std::io::{self, BufWriter, Write};
use std::fs::File;

pub fn writer_graph(graph: NGfa, filename: &str) {
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);

    for node in graph.nodes.iter() {
        write!(f, "{}\t{}\t{}\n", "S", node.0, node.1.seq);
    }
    for edge in graph.edges.iter() {
        write!(f, "{}\t{}\t{}\t{}\t{}\t{}\n", "L", edge.from, plus_minus(&edge.from_dir), edge.to, plus_minus(&edge.to_dir), "0M");
    }
    for path in graph.paths.iter() {
        let f1: Vec<String> = path.nodes.iter().zip(&path.dir).map(|n| format!("{}{}", n.0, plus_minus(n.1))).collect();
        let f2: Vec<String> = (0..path.nodes.len()).into_iter().map(|n|"*".to_string()).collect();
        write!(f, "{}\t{}\t{}\t{}\n", "P", path.name.clone(), f1.join(","), "*");
    }
}

pub fn plus_minus(dir: &bool) -> String{
    let f = {if *dir{"+".to_string()} else {"-".to_string()}};
    return f
}