use std::arch::x86_64::_rdrand32_step;
use std::collections::HashMap;
use std::fmt::Debug;
use gfa_reader::{Gfa, Path};
use crate::stats::helper::calculate_core;

/// Wrapper for sliding window
///
/// TODO
/// - add different metrics
pub fn sliding_window_wrapper(graph: &Gfa, binsize: u32, steosize: u32) -> Vec<(String, Vec<f64>)>{
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
pub fn sw2(input: Vec<u32>, binsize_input: u32, step: u32) -> Vec<f64>{
    let binsize = binsize_input as usize;
    let mut start = 0;
    let maxsize= input.len();
    let mut result = Vec::new();
    while start < maxsize{
        println!("{}", start);
        println!("{}", start+binsize);
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
    println!("{:?}", v);

    Some(mean)
}



