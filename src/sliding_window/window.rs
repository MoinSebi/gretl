use std::arch::x86_64::_rdrand32_step;
use std::collections::HashMap;
use gfa_reader::{Gfa, Path};
use crate::stats::helper::calculate_core;

/// Wrapper for sliding window
///
/// TODO
/// - add different metrics
pub fn sw_wrapper(graph: &Gfa, binsize: u32, steosize: u32) -> Vec<(String, Vec<u32>)>{
    let mut result = Vec::new();
    let core = calculate_core(graph);

    for path in graph.paths.iter(){
        let vector = make_vector(path, &graph, &core);
        let sww = sw2(vector, binsize, steosize);
        result.push((path.name.clone(), sww));
    }
    return result
}

/// Create the vector for sliding window
pub fn make_vector(path: &Path, graph:&Gfa, core: &HashMap<u32, (u32, u32)>) -> Vec<u32>{
        let mut vv = Vec::new();
        for node in path.nodes.iter(){
            let size = graph.nodes.get(node).unwrap().len;
            let level = core.get(&node.parse::<u32>().unwrap()).unwrap().1;
            for x in (0..size){
                vv.push(level);
            }
        }

    vv
}


/// Sliding window
pub fn sw2(input: Vec<u32>, binsize: u32, step: u32) -> Vec<u32>{
    let binsize = binsize as usize;
    let mut start = 0;
    let maxsize= input.len();
    let mut result = Vec::new();
    while start < maxsize{
        let f: u32= input[start..start+binsize].iter().sum();
        result.push(f);
        start += step as usize;
    }
    result
}