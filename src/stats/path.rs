use std::collections::{HashMap, HashSet};
use gfa_reader::{Node, Path, Gfa};
use crate::stats::helper::{calculate_core, calculate_depth, mean, median};


pub fn path_stats_wrapper(graph: &Gfa) -> Vec<(String, Vec<String>)>{
    let mut res = Vec::new();
    let core = calculate_core(&graph);
    let depth = calculate_depth(&graph);
     for p in graph.paths.iter(){
         let mut result = Vec::new();

         result.push(path_seq_len(p, &graph.nodes).to_string());
         result.push(path_node_len(&p.nodes).to_string());
         result.push(path_unique(p).to_string());

         let (v,m) = path_jumps(p);
         result.push(v.to_string());
         result.push(m.to_string());

         result.push(path_jumps_2(p, None).to_string());

         result.push(mean_depth(p, &depth).to_string());
         result.push(median_depth(p, &depth).to_string());
         result.push(mean_sim(p, &depth).to_string());
         result.push(median_sim(p, &depth).to_string());

         res.push((p.name.to_string(), result))
     }

    res
}


/// Count the number of nodes for each path
pub fn path_node_len(path: &Vec<String>) -> usize{
    path.len()
}


/// Calculate the length of path
pub fn path_seq_len(path: &Path, nodes: &HashMap<String, Node>) -> usize{
    let mut size = 0;
    for x in path.nodes.iter(){
        size += nodes.get(x).unwrap().len;
    }
    return size
}


/// Count the number of inverted nodes for each path
pub fn path_node_inverted(path: &Path) -> usize{
    path.dir.iter().filter(|&n | *n == true).count()
}

/// Count the number of inverted nodes for each path
pub fn path_seq_inverted(path: &Path, nodes: HashMap<String, Node>) -> usize{
    let sums: usize = path.dir.iter().zip(&path.nodes).filter(|&n | *n.0 == true).map(|s| nodes.get(s.1).unwrap().len).sum();
    return sums
}


/// Calculate the total number of jumps
///
/// Return:
/// - total number of jumps
/// - total number of jumps divided by the number of nodes
pub fn path_jumps(path: &Path) -> (usize, f64){
    let mut c: i64 = 0;
    let mut last = 0;
    for x in path.nodes.iter(){
        let u: u32 = x.parse().unwrap();
        c += (u as i64- last as i64).abs();
        last = u
    }


    return (c as usize, c as f64/path.nodes.len() as f64)
}

/// Calculate the count of how many jumps are bigger than X
pub fn path_jumps_2(path: &Path, val: Option<i32> ) -> u32{
    let distance = val.unwrap_or(20);
    let mut c: u32 = 0;
    let mut last = 0;
    for x in path.nodes.iter(){
        let u: u32 = x.parse().unwrap();
        let ff: i32 = u as i32 - last as i32;
        if ff.abs() > distance{
            c += 1
        }
    }
    return c
}


pub fn path_unique(path: &Path) -> usize{
    let hp: HashSet<String> = path.nodes.iter().cloned().collect();
    return hp.len()
}


/// Calculate the number of repeated nodes
pub fn path_cycle(path: &Path){
    let mut c = 0;
    let mut hs: HashSet<&String> = HashSet::new();
    for x in path.nodes.iter(){
        if hs.contains(x){
            c += 1
        }
        hs.insert(x);
    }
}


pub fn mean_depth(path: &Path, count: &HashMap<u32, u32> ) -> f64{
    let mut data = Vec::new();
    for x in path.nodes.iter(){
        data.push(count.get(&x.parse::<u32>().unwrap()).unwrap().clone())
    }
    mean(&data)
}

pub fn median_depth(path: &Path, count: &HashMap<u32, u32> ) -> u32{
    let mut data = Vec::new();
    for x in path.nodes.iter(){
        data.push(count.get(&x.parse::<u32>().unwrap()).unwrap().clone())
    }
    median(&mut data)
}


pub fn mean_sim(path: &Path, count: &HashMap<u32, u32> ) -> f64{
    let mut data = Vec::new();
    for x in path.nodes.iter(){
        data.push(count.get(&x.parse::<u32>().unwrap()).unwrap().clone())
    }
    mean(&data)
}

pub fn median_sim(path: &Path, count: &HashMap<u32, u32> ) -> u32{
    let mut data = Vec::new();
    for x in path.nodes.iter(){
        data.push(count.get(&x.parse::<u32>().unwrap()).unwrap().clone())
    }
    median(&mut data)
}