use crate::node_list::writer::write_wrapper;
use gfa_reader::{Gfa, Pansn};

/// Wrapper function for node list analysis
///
pub fn wrapper_node_list(
    graph: &Gfa<u32, (), ()>,
    wrapper: &Pansn<u32, (), ()>,
    filename: &str,
    feature_bitvec: &Vec<bool>,
) -> Result<(), Box<dyn std::error::Error>> {
    let _paths = wrapper.get_path_genome();
    let (mask, offset) = off_setter(graph);
    let mut result_vec = vec![[0; 4]; mask.len()];

    if feature_bitvec[0] {
        let node_len = calc_node_len2(graph, mask.len(), offset);
        merge_vec(&mut result_vec, &node_len, 0);
    }
    if feature_bitvec[1] {
        let node_len = similarity_mask(graph, mask.len(), offset);
        merge_vec(&mut result_vec, &node_len, 1);
    }
    if feature_bitvec[2] {
        let node_len = depth_mask(graph, mask.len(), offset);
        merge_vec(&mut result_vec, &node_len, 2);
    }
    if feature_bitvec[3] {
        let node_len = node_degree_max(graph, mask.len(), offset);
        merge_vec(&mut result_vec, &node_len, 3);
    }

    write_wrapper(
        &result_vec,
        feature_bitvec,
        &mask,
        vec!["Length", "Similarity", "Depth", "ND"],
        filename,
        graph,
    )?;
    Ok(())
}

pub fn merge_vec(vec1: &mut Vec<[u32; 4]>, vec2: &Vec<u32>, c: usize) {
    for (i, x) in vec2.iter().enumerate() {
        vec1[i][c] = *x;
    }
}

/// Compute the node len
///
/// Return a vector
pub fn calc_node_len2(graph: &Gfa<u32, (), ()>, len: usize, offset: usize) -> Vec<u32> {
    let mut result = vec![0; len];

    for node in graph.segments.iter() {
        result[node.id as usize - offset] = node.sequence.get_len() as u32;
    }
    result
}

/// Compute the node len
///
/// Return a vector
pub fn depth_mask(graph: &Gfa<u32, (), ()>, len: usize, offset: usize) -> Vec<u32> {
    let mut result = vec![0; len];

    for x in graph.paths.iter() {
        for y in x.nodes.iter() {
            result[*y as usize - offset] += 1;
        }
    }
    result
}

/// Compute the node len
///
/// Return a vector
pub fn similarity_mask(graph: &Gfa<u32, (), ()>, len: usize, offset: usize) -> Vec<u32> {
    let mut result = vec![0; len];

    for x in graph.paths.iter() {
        let mut oo: Vec<u32> = x.nodes.clone();
        oo.sort();
        oo.dedup();
        for y in oo.iter() {
            result[*y as usize - offset] += 1;
        }
    }
    result
}

pub fn node_degree_max(graph: &Gfa<u32, (), ()>, len: usize, offset: usize) -> Vec<u32> {
    let mut result = vec![0; len];

    for x in graph.links.iter() {
        let fromu: usize = x.from as usize;
        let tou: usize = x.to as usize;
        result[fromu - offset] += 1;
        result[tou - offset] += 1;
    }
    result
}

pub fn off_setter(graph: &Gfa<u32, (), ()>) -> (Vec<bool>, usize) {
    let offset = graph.segments[0].id as usize;
    let maxx = graph.segments[graph.segments.len() - 1].id as usize;
    let mut result = vec![false; maxx - offset + 1];

    for node in graph.segments.iter() {
        result[node.id as usize - offset] = true;
    }
    (result, offset)
}
