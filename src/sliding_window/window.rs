use crate::helpers::helper::{calculate_similarity, node_len};
use crate::sliding_window::sliding_window_main::Metric;
use gfa_reader::{NCGfa, NCPath, Pansn};
use std::fmt::Debug;

/// Wrapper for sliding window
///
/// TODO
/// - add different metrics
pub fn sliding_window_wrapper(
    graph: &NCGfa<()>,
    wrapper: &Pansn<NCPath>,
    binsize: u32,
    steosize: u32,
    metric: Metric,
    node: bool,
) -> Vec<(String, Vec<f64>)> {
    let mut paths = wrapper.get_path_genome();

    let mut result = Vec::new();
    let mut core = calculate_similarity(&paths, graph);
    match metric {
        Metric::Nodesizem => core = node_len(graph),
        Metric::Similarity => {}
    }

    let node_len = node_len(graph);
    for path in graph.paths.iter() {
        let vector = make_vector(path, &node_len, &core, &node);
        let sww = sliding_window(vector, binsize, steosize);
        result.push((path.name.clone(), sww));
    }
    return result;
}

/// Create the vector for sliding window
pub fn make_vector(path: &NCPath, node_len: &Vec<u32>, core: &Vec<u32>, node: &bool) -> Vec<u32> {
    let mut vv = Vec::new();
    if *node {
        for n in path.nodes.iter() {
            let level = core[*n as usize];
            vv.push(level.clone());
        }
    } else {
        for n in path.nodes.iter() {
            let size = node_len[*n as usize - 1];
            let level = core[*n as usize - 1];
            for _x in 0..size {
                vv.push(level.clone());
            }
        }
    }
    vv
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
        mean += (value.into() - mean) / f64::from(count + 1.0);
        count += 1.0;
    }

    Some(mean)
}
