use std::collections::{HashSet};
use gfa_reader::{GraphWrapper, NCGfa, NCPath};
use crate::helpers::helper::calculate_similarity;


/// Compute the amount of sequence in each similarity level
pub fn accession2level(graph: &NCGfa<()>, wrapper: &GraphWrapper<NCPath>) -> Vec<(String, Vec<(u32, u32)>)>{
    let cores = calculate_similarity(wrapper, graph);
    let metric_maxval = cores.iter().max().unwrap();
    let mut res = Vec::new();

    for (name, p) in wrapper.genomes.iter() {
        let mut depth: Vec<(u32, u32)> = vec![(0,0); *metric_maxval as usize];

        let mut path_nodes: HashSet<&u32> = HashSet::new();
        for path in p.iter() {
            for node in path.nodes.iter() {
                path_nodes.insert(node);
            }
        }
        for x in path_nodes.iter() {
            let metric_value = cores[**x as usize-1] as usize;

            depth[metric_value as usize-1].0 += 1;
            depth[metric_value as usize-1].1 += graph.nodes[**x as usize -1].seq.len() as u32;
        }
        res.push((name.clone(), depth));

    }


    return res
}

