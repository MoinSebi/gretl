use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::vec;
use gfa_reader::Gfa;
use crate::node_list::writer::{make_buffer, write_header, write_list};

use std::io::{BufWriter, Write};
use std::fs::File;

/// This is wrapper a various stats that can be computed with GFA
pub fn wrapper_node(graph: &Gfa, filename: &str, what: Vec<&str>){

    let mut node_values = graph.nodes.iter().map(|n| n.0).collect::<Vec<&String>>();
    let all_digits = node_values.iter().all(|s| s.chars().all(|c| c.is_digit(10)));
    if all_digits {
        node_values.sort_by(|a, b| a.parse::<u32>().unwrap().cmp(&b.parse::<u32>().unwrap()));
    }
    let mut ff = make_buffer(filename);


    write_header(&node_values, &mut ff);
    if what.contains(&"Length"){
        let len = node_len(graph);
        write_list( ("Length", len), &node_values, &mut ff);
    } if what.contains(&"Core"){
        let core = core12(graph);
        write_list(("Core", core), &node_values, &mut ff);
    } if what.contains(&"Depth"){
        let depth2 = depth12(graph);
        write_list(("Depth", depth2), &node_values, &mut ff);
    } if what.contains(&"ND"){
        let a = node_degree(graph);
        let a1 = make1(&a, 0);
        let a2 = make1(&a, 0);
        let a3 = make1(&a, 0);

        write_list(("ND_out", a1), &node_values,  &mut ff);
        write_list(("ND_in", a2), &node_values, &mut ff);
        write_list(("ND_in", a3), &node_values, &mut ff);
    }
}

/// This is wrapper a various stats that can be computed with GFA
/// TODO
/// - negative (both sides)
/// - sizes (size, size difference)
/// - id distance
/// - order by first then by second
pub fn wrapper_edge(graph: &Gfa, filename: &str, what: Vec<&str>){

    let mut node_values = graph.nodes.iter().map(|n| n.0).collect::<Vec<&String>>();
    let all_digits = node_values.iter().all(|s| s.chars().all(|c| c.is_digit(10)));
    if all_digits {
        node_values.sort_by(|a, b| a.parse::<u32>().unwrap().cmp(&b.parse::<u32>().unwrap()));
    }
    let mut ff = make_buffer(filename);


    write_header(&node_values, &mut ff);
    if what.contains(&"Length"){
        let len = node_len(graph);
        write_list( ("Length", len), &node_values, &mut ff);
    } if what.contains(&"Core"){
        let core = core12(graph);
        write_list(("Core", core), &node_values, &mut ff);
    } if what.contains(&"Depth"){
        let depth2 = depth12(graph);
        write_list(("Depth", depth2), &node_values, &mut ff);
    } if what.contains(&"ND"){
        let a = node_degree(graph);
        let a1 = make1(&a, 0);
        let a2 = make1(&a, 0);
        let a3 = make1(&a, 0);

        write_list(("ND_out", a1), &node_values,  &mut ff);
        write_list(("ND_in", a2), &node_values, &mut ff);
        write_list(("ND_in", a3), &node_values, &mut ff);
    }
}



pub fn make1<'a>(hm: &HashMap<&'a String, [u32; 3]>, f: usize) -> HashMap<&'a String, u32>{
    let mut hashmap1: HashMap<&String, u32> = HashMap::new();
    for x in hm.iter(){
        hashmap1.insert(*x.0, x.1[f]);
    }
    hashmap1
}

// Get the x's value of a (u32, u32, u32)




/// Compute the node len
///
/// Return a vector
pub fn node_len(graph: &Gfa) ->  HashMap<&String, u32>{
    let mut result = HashMap::new();

    for (id, node) in graph.nodes.iter(){
        result.insert(id, node.len as u32);
    }
    return result
}

pub fn depth12(graph: &Gfa) -> HashMap<&String, u32>{
    let mut depth = HashMap::new();
    for i in graph.nodes.iter() {
        depth.insert(i.0, 0);
    }
    for p in graph.paths.iter(){
        for y in p.nodes.iter(){
            *depth.get_mut(y).unwrap() += 1;
        }
    }
    depth

}

pub fn core12(graph: &Gfa) -> HashMap<&String, u32>{
    let mut depth = HashMap::new();
    for i in graph.nodes.iter() {
        depth.insert(i.0, 0);
    }
    for p in graph.paths.iter(){
        let v: HashSet<_> = p.nodes.iter().cloned().collect();
        for y in p.nodes.iter(){
            *depth.get_mut(y).unwrap() += 1;
        }
    }
    depth

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

pub fn node_degree(graph: &Gfa) -> HashMap<&String, [u32; 3]> {


    // degree (total, in, out)
    let mut degree: HashMap<&String, [u32; 3]> = HashMap::new();
    for i in graph.nodes.iter() {
        degree.insert(i.0, [0, 0, 0]);
    }
    for edge in graph.edges.iter() {
        degree.get_mut(&edge.from).unwrap()[0] += 1;
        degree.get_mut(&edge.from).unwrap()[2] += 1;

        degree.get_mut(&edge.to).unwrap()[0] += 1;
        degree.get_mut(&edge.to).unwrap()[1] += 1;
    }

    return degree
}