use std::collections::{HashMap, HashSet};
use gfa_reader::{Node, Path, Gfa, NCPath, NCNode, GraphWrapper, NCGfa};
use crate::helpers::helper::{calculate_core, calculate_depth, node_degree};
use crate::stats::helper::{mean, meanf, median};
use crate::stats::path_stats::Arithmetic::MEDIAN;


/// Wrapper for path statistics
pub fn path_stats_wrapper(graph: &NCGfa<()>, gw: &GraphWrapper<NCPath>) -> Vec<(String, Vec<String>)>{

    // Total results
    let mut res = Vec::new();

    // Calculate similarity
    let core = calculate_core(&gw, graph);

    // Calculate node degree
    let test = node_degree(&graph);

    // Calculate depth
    let depth = calculate_depth(&gw, graph);

    // Iterate over all paths and calculate statistics
     for path in graph.paths.iter(){
         // Temporary results for each path
         let mut result_temp = Vec::new();

         // Amount of sequence and number of nodes in the path + number of unique nodes
         result_temp.push(path_seq_len(path, &graph.nodes).to_string());
         result_temp.push(path_node_len(&path.nodes).to_string());
         result_temp.push(path_unique(path).to_string());

         // Number of inverted nodes + their sequence
         result_temp.push(path_node_inverted(path).to_string());
         result_temp.push(path_seq_inverted(path, &graph.nodes).to_string());

         // Number of jumps - normalized + bigger than x
         let (jumps_total, jumps_normalized) = path_jumps(path);
         result_temp.push(jumps_total.to_string());
         result_temp.push(jumps_normalized.to_string());


         let jumps_bigger_than_x = path_jumps_bigger(path, None);
         result_temp.push(jumps_bigger_than_x.to_string());

         let mean_depth = mean_path_hm(path, &depth, Arithmetic::MEAN);
         let median_depth = mean_path_hm(path, &depth, Arithmetic::MEDIAN);
         let mean_similarity = mean_path_hm(path, &core, Arithmetic::MEAN);
         let median_similarity = mean_path_hm(path, &core, Arithmetic::MEDIAN);

         // Add to temporary result
         result_temp.push(mean_depth.to_string());
         result_temp.push(median_depth.to_string());
         result_temp.push(mean_similarity.to_string());
         result_temp.push(median_similarity.to_string());


         result_temp.push(mean_path_hm(path, &test.2, Arithmetic::MEAN).to_string());
         result_temp.push("test".to_string());



         res.push((path.name.to_string(), result_temp))


     }

    res
}


/// Count the number of nodes for each path
pub fn path_node_len(path: &Vec<u32>) -> usize{
    path.len()
}


/// Calculate the length of path
pub fn path_seq_len(path: &NCPath, nodes: &Vec<NCNode<()>>) -> usize{
    let mut size = 0;
    for x in path.nodes.iter(){
        size += nodes[*x as usize -1].seq.len();
    }
    return size
}

#[allow(dead_code)]
/// Count the number of inverted nodes for each path
pub fn path_node_inverted(path: &NCPath) -> usize{
    path.dir.iter().filter(|&n | *n == false).count()
}

#[allow(dead_code)]
/// Count the number of inverted nodes for each path
pub fn path_seq_inverted(path: &NCPath, nodes: &Vec<NCNode<()>>) -> usize{
    let sums: usize = path.dir.iter().zip(&path.nodes).filter(|&n | *n.0 == false).map(|s| nodes.get(*s.1 as usize).unwrap().seq.len()).sum();
    return sums
}


/// Calculate the total number of jumps
///
/// Return:
/// - total number of jumps
/// - total number of jumps divided by the number of nodes
///
/// TODO
/// - running average (no overflow)
pub fn path_jumps(path: &NCPath) -> (usize, f64){
    let mut c: i64 = 0;
    let mut last = 0;
    for x in path.nodes.iter(){
        let u: u32 = *x;
        c += (u as i64- last as i64).abs();
        last = u

    }
    return (c as usize, c as f64/path.nodes.len() as f64)
}

/// Count the number of jumps bigger than X
pub fn path_jumps_bigger(path: &NCPath, val: Option<i32> ) -> u32{
    let distance = val.unwrap_or(20);
    let mut c: u32 = 0;
        let mut last = 0;
        for x in path.nodes.iter() {
            let ff: i32 = *x as i32 - last as i32;
            last = *x;
            if ff.abs() > distance {
                c += 1
            }
        }

    return c
}

/// Number of unique nodes in a path
pub fn path_unique(path: &NCPath) -> usize{
    let hp: HashSet<u32> = path.nodes.iter().cloned().collect();
    return hp.len()
}

#[allow(dead_code)]
/// Calculate the number of repeated nodes
pub fn path_cycle(path: &NCPath) -> usize{
    let mut _c = 0;
    let mut hs: HashSet<&u32> = HashSet::new();
    for x in path.nodes.iter(){
        if hs.contains(x){
            _c += 1
        }
        hs.insert(x);
    }
    _c
}

pub enum Arithmetic {
    MEAN,
    MEDIAN,
}

pub fn mean_path_hm(path: &NCPath, count: &Vec<u32>, ari: Arithmetic) -> f64{
    let mut data = Vec::new();
    for x in path.nodes.iter(){
        data.push(count[*x as usize - 1])
    }
    let mut result: f64 = 0.0;
    match ari {
        Arithmetic::MEAN =>  result = mean(&data),
        _ => result = median(&mut data),
    }
    result
}
