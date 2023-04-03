use std::collections::{HashMap, HashSet};
use gfa_reader::{Node, Path, Gfa};


pub fn path_stats_wrapper(graph: &Gfa) -> Vec<(&str, String)>{
    let mut result = Vec::new();
     for x in graph.paths.iter(){
        let d = path_number_inverted(x);
     }
    result.push(("test", "test".to_string()));
    result
}

/// Count the number of inverted nodes for each path
///
///
pub fn path_number_inverted(path: &Path) -> usize{
    path.dir.iter().filter(|&n | *n == true).count()
}


/// Count the number of inverted nodes for each path
///
///
pub fn path_len(path: Path) -> usize{
    path.nodes.len()
}

pub fn path_unique(path: Path) -> usize{
    let hp: HashSet<String> = path.nodes.iter().cloned().collect();
    return hp.len()
}


/// Calculate the
pub fn path_jumps(path: Path) -> (usize, f64){
    let mut c = 0;
    let mut last = 0;
    for x in path.nodes.iter(){
        let u: u32 = x.parse().unwrap();
        c += u - last;
        last = u
    }


    return (c as usize, c as f64/path.nodes.len() as f64)
}


pub fn path_jumps_2(path: Path, val: Option<i32> ) -> u32{
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


/// Calculate the length of path
pub fn path_length(path: Path, nodes: HashMap<String, Node>) -> usize{
    let mut size = 0;
    for x in path.nodes.iter(){
        size += nodes.get(x).unwrap().len;
    }
    return size
}


/// Calculate the number of repeated nodes
pub fn path_cycle(path: Path){
    let mut c = 0;
    let mut hs: HashSet<&String> = HashSet::new();
    for x in path.nodes.iter(){
        if hs.contains(x){
            c += 1
        }
        hs.insert(x);
    }
}