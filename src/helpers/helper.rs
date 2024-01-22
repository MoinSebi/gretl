use gfa_reader::{NCGfa, NCPath, Pansn};
use std::collections::HashSet;
use std::fmt::Debug;

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
        mean += (value.into() - mean.clone()) / f64::from(count.clone() + 1.0);
        count += 1.0;
    }

    Some(mean)
}

/// Counting the amount of accessions and depth
pub fn calculate_depth(wrapper: &Vec<(String, Vec<&NCPath>)>, graph: &NCGfa<()>) -> Vec<u32> {
    let mut depth: Vec<u32> = vec![0; graph.nodes.len()];
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
pub fn calculate_similarity(wrapper: &Vec<(String, Vec<&NCPath>)>, graph: &NCGfa<()>) -> Vec<u32> {
    let mut depth: Vec<u32> = vec![0; graph.nodes.len()];
    for p in wrapper.iter() {
        let mut path_nodes: HashSet<&u32> = HashSet::new();
        for path in p.1.iter() {
            for node in path.nodes.iter() {
                path_nodes.insert(node);
            }
        }
        for x in path_nodes.iter() {
            depth[**x as usize - 1] += 1;
        }
    }
    depth.shrink_to_fit();
    depth
}

/// Calculate node degree (in, out, total)
pub fn node_degree(graph: &NCGfa<()>) -> (Vec<u32>, Vec<u32>, Vec<u32>) {
    let mut degree_in: Vec<u32> = vec![0; graph.nodes.len()];
    let mut degree_out: Vec<u32> = vec![0; graph.nodes.len()];
    let mut degree_total: Vec<u32> = vec![0; graph.nodes.len()];
    for x in graph.edges.as_ref().unwrap().iter() {
        let fromu: usize = x.from.clone() as usize;
        let tou: usize = x.to.clone() as usize;

        degree_out[fromu - 1] += 1;
        degree_in[tou - 1] += 1;
        degree_total[fromu - 1] += 1;
        degree_total[tou - 1] += 1;
    }
    return (degree_in, degree_out, degree_total);
}

/// Calculate node degree (in, out, total)
pub fn node_degree_total(graph: &NCGfa<()>) -> Vec<u32> {
    let mut degree_total: Vec<u32> = vec![0; graph.nodes.len()];
    for x in graph.edges.as_ref().unwrap().iter() {
        let fromu: usize = x.from.clone() as usize;
        let tou: usize = x.to.clone() as usize;
        degree_total[fromu - 1] += 1;
        degree_total[tou - 1] += 1;
    }
    return degree_total;
}

/// Compute the node len
///
/// Return a vector
pub fn node_len(graph: &NCGfa<()>) -> Vec<u32> {
    let mut result = Vec::new();

    for node in graph.nodes.iter() {
        result.push(node.seq.len() as u32);
    }
    return result;
}
