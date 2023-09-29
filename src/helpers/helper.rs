use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::path::Path;
use gfa_reader::{Edge, Gfa, GraphWrapper, NCGfa, NCPath};

fn calculate_average<T>(v: &[T]) -> Option<f64>
    where T:  Into<f64> + Copy + Debug
{
    if v.is_empty() {
        return None;
    }

    let mut mean: f64 = f64::from(0);
    let mut count: f64 = 0.0;

    for &value in v {
        mean += ((value.into() - mean.clone())/ f64::from(count.clone() + 1.0));
        count += 1.0;
    }

    Some(mean)
}

/// Counting the amount of accessions and depth
pub fn calculate_depth(wrapper: &GraphWrapper<NCPath>, graph: &NCGfa<()>) -> Vec<u32>{
    let mut depth: Vec<u32> = vec![0; graph.nodes.len()];
    for (name, p) in wrapper.genomes.iter() {
        for path in p.iter() {
            for x in path.nodes.iter() {
                depth[*x as usize-1] += 1;
            }
        }
    }
    depth.shrink_to_fit();
    depth
}

/// Counting the amount of accessions and depth
pub fn calculate_core(wrapper: &GraphWrapper<NCPath>, graph: &NCGfa<()> ) -> Vec<u32>{
    let mut depth: Vec<u32> = vec![0; graph.nodes.len()];
    for (name, p) in wrapper.genomes.iter() {
        let mut path_nodes: HashSet<&u32> = HashSet::new();
        for path in p.iter() {
            for node in path.nodes.iter() {
                path_nodes.insert(node);
            }
        }
        for x in path_nodes.iter() {
            depth[**x as usize-1] += 1;
        }

    }
    depth.shrink_to_fit();
    depth
}


/// Calculate node degree (in, out, total)
pub fn node_degree(graph: &NCGfa<()>) -> (Vec<u32>, Vec<u32>, Vec<u32>){
    let mut degree_in: Vec<u32> = vec![0; graph.nodes.len()];
    let mut degree_out: Vec<u32> = vec![0; graph.nodes.len()];
    let mut degree_total: Vec<u32> = vec![0; graph.nodes.len()];
    for x in graph.edges.as_ref().unwrap().iter(){
        let fromu: usize = x.from.clone() as usize;
        let tou: usize = x.to.clone() as usize;

        degree_out[fromu-1] += 1;
        degree_in[tou-1] += 1;
        degree_total[fromu-1] += 1;
        degree_total[tou-1] += 1;

    }
    return (degree_in, degree_out, degree_total)

}

/// Calculate node degree (in, out, total)
pub fn node_degree_total(graph: &NCGfa<()>) -> Vec<u32>{
    let mut degree_total: Vec<u32> = vec![0; graph.nodes.len()];
    for x in graph.edges.as_ref().unwrap().iter(){
        let fromu: usize = x.from.clone() as usize;
        let tou: usize = x.to.clone() as usize;
        degree_total[fromu-1] += 1;
        degree_total[tou-1] += 1;

    }
    return degree_total

}

/// Compute the node len
///
/// Return a vector
pub fn node_len(graph: &NCGfa<()>) ->  Vec<u32>{
    let mut result = Vec::new();

    for node in graph.nodes.iter(){
        result.push(node.seq.len() as u32);
    }
    return result
}

pub fn transpose_matrix<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let cols = matrix[0].len();

    let transposed: Vec<Vec<T>> = (0..cols)
        .map(|j| matrix.iter().map(|row| row[j].clone()).collect())
        .collect();

    transposed
}

