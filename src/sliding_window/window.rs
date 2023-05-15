use std::arch::x86_64::_rdrand32_step;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use gfa_reader::{Gfa, Path};
use crate::node_list::wrapper::make_mapper;
use crate::stats::helper::calculate_core;

/// Wrapper for sliding window
///
/// TODO
/// - add different metrics
pub fn sliding_window_wrapper(graph: &Gfa, binsize: u32, steosize: u32, metric: &str) -> Vec<(String, Vec<f64>)>{
    let mut result = Vec::new();
    let mut core = calculate_core12(graph);
    if metric == "length"{
        core = node_len(graph);
    }
    for path in graph.paths.iter(){
        let vector = make_vector(path, &graph, &core);
        let sww = sw2(vector, binsize, steosize);
        result.push((path.name.clone(), sww));
    }
    return result
}

/// Create the vector for sliding window
pub fn make_vector(path: &Path, graph:&Gfa, core: &HashMap<u32, u32>) -> Vec<u32>{
        let mut vv = Vec::new();
        for node in path.nodes.iter(){
            let size = graph.nodes.get(node).unwrap().len;
            let level = core.get(&node.parse::<u32>().unwrap()).unwrap();
            for x in (0..size){
                vv.push(level.clone());
            }
        }

    vv
}

/// Counting the amount
pub fn calculate_core12(graph: &Gfa) -> HashMap<u32, u32>{

    // Initialization hashmap
    let mut count: HashMap<u32,u32> = HashMap::new();
    for x in &graph.nodes{
        count.insert(x.0.parse().unwrap(),  0);
    }

    // Calculate the amount of sequence and the amount of nodes
    for x in &graph.paths{
        // Count every node only once
        let v: HashSet<_> = x.nodes.iter().cloned().collect();
        for y in v{
            *count.get_mut(&y.parse().unwrap()).unwrap() += 1;
        }
    }
    count.shrink_to_fit();
    count
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

/// Compute the node len
///
/// Return a vector
pub fn node_len(graph: &Gfa) ->  HashMap<u32, u32>{
    let mapper = make_mapper(graph);

    let mut result = HashMap::new();
    for (id, node) in graph.nodes.iter(){
        let index = mapper.get(id).unwrap();
        result.insert(index.clone() as u32, node.len as u32);
    }
    return result
}


