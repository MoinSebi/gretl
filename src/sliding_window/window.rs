/*
TODO
- metric
- sizes
- moving
 */

use std::collections::HashMap;
use gfa_reader::{Gfa, Path};
use crate::stats::helper::calculate_core;

pub fn sw_wrapper(graph: &Gfa){
    let mut result = Vec::new();
    let core = calculate_core(graph);

    for path in graph.paths.iter(){
        let vector = make_vector(path, &graph, &core);
        let sww = sw2(vector);
        result.push(sww);
    }
}

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

pub fn sw2(input: Vec<u32>) -> Vec<u32>{
    let binsize = 500000;
    let mut start = 0;
    let maxsize= input.len();
    let mut result = Vec::new();
    while start < maxsize{
        let f: u32= input[start..start+binsize].iter().sum();
        result.push(f);




        start += binsize;
    }
    result
}