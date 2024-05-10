use crate::helpers::helper::{calc_depth, calc_similarity, calc_node_len};
use crate::sliding_window::sliding_window_main::Metric;
use gfa_reader::{Gfa, Pansn, Path};
use std::fmt::Debug;

/// Wrapper for sliding window
///
/// TODO
/// - add different metrics
pub fn sliding_window_wrapper(
    graph: &Gfa<u32, (), ()>,
    wrapper: &Pansn<u32, (), ()>,
    binsize: u32,
    stepsize: u32,
    metric: Metric,
    node: bool,
) -> Vec<(String, Vec<f64>)> {
    let paths = wrapper.get_path_genome();

    let mut result = Vec::new();
    let mut core = calc_similarity(&paths, graph);
    match metric {
        Metric::Nodesizem => core = calc_node_len(graph),
        Metric::Depth => core = calc_depth(&paths, graph),
        Metric::Similarity => {}
    }

    let node_len = calc_node_len(graph);
    for path in graph.paths.iter() {
        let vector = path2metric_vector(path, &node_len, &core, &node);
        let sww = sliding_window(vector, binsize, stepsize);
        result.push((path.name.clone(), sww));
    }
    result
}

/// Create the vector for sliding window
pub fn path2metric_vector(
    path: &Path<u32, (), ()>,
    node_len: &Vec<u32>,
    core: &Vec<u32>,
    node: &bool,
) -> Vec<u32> {
    let mut metric_vector = Vec::new();
    if *node {
        for n in path.nodes.iter() {
            let level = core[*n as usize];
            metric_vector.push(level);
        }
    } else {
        for n in path.nodes.iter() {
            let size = node_len[*n as usize];
            let level = core[*n as usize];
            for _x in 0..size {
                metric_vector.push(level);
            }
        }
    }
    metric_vector
}

/// Sliding window
pub fn sliding_window(input: Vec<u32>, binsize_input: u32, step: u32) -> Vec<f64> {
    let binsize = binsize_input as usize;
    let mut start = 0;
    let maxsize = input.len();
    let mut result = Vec::new();
    while start < maxsize {
        let a = &start + &binsize;
        if a > maxsize {
            let f: f64 = calculate_average(&input[start..maxsize]).unwrap();
            result.push(f);
            break;
        }
        let f: f64 = calculate_average(&input[start..a]).unwrap();

        result.push(f);
        start += step as usize;
    }
    result
}

fn calculate_average<T>(v: &[T]) -> Option<f64>
where
    T: Into<f64> + Copy + Debug,
{
    if v.is_empty() {
        return None;
    }

    let mut mean: f64 = f64::from(0);
    let mut count: f64 = 0.0;

    for &value in v {
        mean += (value.into() - mean) / (count + 1.0);
        count += 1.0;
    }

    Some(mean)
}
