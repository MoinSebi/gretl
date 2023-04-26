use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::vec;
use gfa_reader::Gfa;
use crate::node_list::writer::{make_buffer, write_header, write_list};

use std::io::{BufWriter, Write};
use std::fs::File;

/// This is wrapper a various stats that can be computed with GFA
pub fn wrapper(graph: &Gfa, filename: &str, what: Vec<&str>){
    let mapper = make_mapper(graph);

    let mut ff = make_buffer(filename);


    let len = node_len(graph, &mapper);
    let (core, depth) = core_depth(graph, &mapper);
    let (t,z,u) = node_degree(graph, &mapper);
    write_header(&mapper, &mut ff);
    if what.contains(&"Length"){
        write_list(("Length", len), &mut ff);
    } if what.contains(&"Core"){
        write_list(("Core", core), &mut ff);
    } if what.contains(&"Depth"){
        write_list(("Depth", depth), &mut ff);
    } if what.contains(&"ND_out"){
        write_list(("ND_out", t), &mut ff);
    } if what.contains(&"ND_in"){
        write_list(("ND_in", z), &mut ff);
    } if what.contains(&"ND_total") {
        write_list(("ND_total", u), &mut ff);
    }


}

/// This is try to make a mapper
pub fn make_mapper(graph: &Gfa) -> HashMap<&String, usize>{
    let mut mapper = HashMap::new();
    for (index, (id, node)) in graph.nodes.iter().enumerate(){
        mapper.insert(id, index);
    }
    mapper
}

/// Compute the node len
///
/// Return a vector
pub fn node_len(graph: &Gfa, mapper: &HashMap<&String, usize>) ->  Vec<u32>{
    let mut result = vec![0; graph.nodes.len()];
    for (id, node) in graph.nodes.iter(){
        let index = mapper.get(id).unwrap();
        result[*index] = node.len as u32;
    }
    return result
}


/// Compute the similarity and depth
pub fn core_depth(graph: &Gfa, mapper: &HashMap<&String, usize>) -> (Vec<u32>, Vec<u32>){
    let mut result = vec![0; graph.nodes.len()];
    let mut result2 = vec![0; graph.nodes.len()];


    for p in graph.paths.iter(){
        let v: HashSet<_> = p.nodes.iter().cloned().collect();
        for y in v.iter(){
            let index = mapper.get(y).unwrap();
            *result.get_mut(*index).unwrap() += 1;
        }
        for y in p.nodes.iter(){
            let index = mapper.get(y).unwrap();
            *result2.get_mut(*index).unwrap() += 1;
        }

    }
    (result, result2)
}

pub fn node_degree(graph: &Gfa, mapper: &HashMap<&String, usize>) -> (Vec<u32>,Vec<u32> ,Vec<u32> ){
    let mut degree_in: Vec<u32> = vec![0; graph.nodes.len()];
    let mut degree_out: Vec<u32> = vec![0; graph.nodes.len()];
    let mut degree_total: Vec<u32> = vec![0; graph.nodes.len()];

    for x in graph.edges.iter(){
        let from_node_index = *mapper.get(&x.from).unwrap();
        let to_node_index = *mapper.get(&x.to).unwrap();
        degree_in[to_node_index] += 1;
        degree_out[from_node_index] += 1;
        degree_total[to_node_index] += 1;
        degree_total[from_node_index] += 1;
    }
    return (degree_in, degree_out, degree_total)
}