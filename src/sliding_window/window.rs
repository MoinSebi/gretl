use std::arch::x86_64::_rdrand32_step;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use gfa_reader::{Gfa, Path};
use crate::node_list::wrapper::make_mapper;
use crate::sliding_window::metrics::{calculate_core12, node_len};
use crate::sliding_window::sliding_window_main::metric;
use crate::stats::helper::calculate_core;

/// Wrapper for sliding window
///
/// TODO
/// - add different metrics
pub fn sliding_window_wrapper(graph: &Gfa, binsize: u32, steosize: u32, metric: metric, node: bool) -> Vec<(String, Vec<f64>)>{
    let mut result = Vec::new();
    let mut core = calculate_core12(graph);
    match metric{
        metric::nodesizem => core = node_len(graph),
        metric::similarity => core = calculate_core12(graph),
    }


    for path in graph.paths.iter(){
        let vector = make_vector(path, &graph, &core, &node);
        let sww = sliding_window(vector, binsize, steosize);
        result.push((path.name.clone(), sww));
    }
    return result
}

/// Create the vector for sliding window
pub fn make_vector(path: &Path, graph:&Gfa, core: &HashMap<u32, u32>, node: &bool) -> Vec<u32>{
    let mut vv = Vec::new();
    if *node{
        for n in path.nodes.iter(){
            let level = core.get(&n.parse::<u32>().unwrap()).unwrap();
            vv.push(level.clone());
        }
    } else {
        for n in path.nodes.iter() {
            let size = graph.nodes.get(n).unwrap().len;
            let level = core.get(&n.parse::<u32>().unwrap()).unwrap();
            for x in (0..size) {
                vv.push(level.clone());
            }
        }
    }
    vv
}


/// Sliding window
pub fn sliding_window(input: Vec<u32>, binsize_input: u32, step: u32) -> Vec<f64>{
    let binsize = binsize_input as usize;
    let mut start = 0;
    let maxsize= input.len();
    let mut result = Vec::new();
    while start < maxsize{
        if start+binsize > maxsize{
            let f: u32= input[start..maxsize].iter().sum();
            let f: f64 = calculate_average(&input[start..maxsize]).unwrap();
            result.push(f);
            break
        }
        let f: f64 = calculate_average(&input[start..start+binsize]).unwrap();

        result.push(f);
        start += step as usize;
    }
    result
}

fn calculate_average<T>(v: &[T]) -> Option<f64>
    where T:  Into<f64> + Copy + Debug
{
    if v.is_empty() {
        return None;
    }

    let mut mean: f64 = f64::from(0);
    let mut count: f64 = 0.0;

    for &value in v {
        mean += ((value.into() - mean)/ f64::from(count + 1.0));
        count += 1.0;
    }

    Some(mean)
}


