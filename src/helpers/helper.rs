use std::collections::HashSet;
use std::fmt::Debug;
use gfa_reader::{Gfa, Pansn, Path};
use log::info;

#[allow(dead_code)]
/// Calculate the average
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

/// Counting the amount of accessions and depth
pub fn calculate_depth(wrapper: &Vec<(String, Vec<&Path<u32, (), ()>>)>, graph: &Gfa<u32, (), ()>) -> Vec<u32> {
    let mut depth: Vec<u32> = vec![0; graph.segments.len()];
    for paths in wrapper.iter() {
        for path in paths.1.iter() {
            for x in path.nodes.iter() {
                depth[*x as usize - 1] += 1;
            }
        }
    }
    depth.shrink_to_fit();
    depth
}

/// Counting the amount of accessions and depth
pub fn calculate_similarity(wrapper: &Vec<(String, Vec<&Path<u32, (), ()>>)>, graph: &Gfa<u32, (), ()>) -> Vec<u32> {
    let mut depth: Vec<u32> = vec![0; graph.segments.len()];
    for p in wrapper.iter() {
        let mut path_nodes: Vec<u32> = p.1.iter().map(|x| x.nodes.iter()).flatten().cloned().collect();
        path_nodes.sort();
        path_nodes.dedup();
        for x in path_nodes.iter() {
            depth[*x as usize - 1] += 1;
        }
    }
    depth.shrink_to_fit();
    depth
}

/// Calculate node degree (in, out, total)
pub fn node_degree(graph: &Gfa<u32, (), ()>) -> (Vec<u32>, Vec<u32>, Vec<u32>) {
    let mut degree_in: Vec<u32> = vec![0; graph.segments.len()];
    let mut degree_out: Vec<u32> = vec![0; graph.segments.len()];
    let mut degree_total: Vec<u32> = vec![0; graph.segments.len()];
    for x in graph.links.iter() {
        let fromu: usize = x.from as usize;
        let tou: usize = x.to as usize;

        degree_out[fromu - 1] += 1;
        degree_in[tou - 1] += 1;
        degree_total[fromu - 1] += 1;
        degree_total[tou - 1] += 1;
    }
    (degree_in, degree_out, degree_total)
}

/// Calculate node degree (in, out, total)
pub fn node_degree_total(graph: &Gfa<u32, (), ()>) -> Vec<u32> {
    let mut degree_total: Vec<u32> = vec![0; graph.segments.len()];
    for x in graph.links.iter() {
        let fromu: usize = x.from as usize;
        let tou: usize = x.to as usize;
        degree_total[fromu - 1] += 1;
        degree_total[tou - 1] += 1;
    }
    degree_total
}

/// Compute the node len
///
/// Return a vector
pub fn node_len(graph: &Gfa<u32, (), ()>) -> Vec<u32> {
    let mut result = Vec::new();

    for node in graph.segments.iter() {
        result.push(node.sequence.get_len() as u32);
    }
    result
}
