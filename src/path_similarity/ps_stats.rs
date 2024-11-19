use crate::helpers::helper::calc_similarity;
use gfa_reader::{Gfa, Pansn};
use log::info;
use rayon::prelude::*;


/// Compute the amount of sequence in each similarity level
pub fn accession2level(
    graph: &Gfa<u32, (), ()>,
    wrapper: &Pansn<u32, (), ()>,
    threads: usize,
) -> Vec<(String, Vec<(u32, u32)>)> {
    info!("Accession2level");
    let paths = wrapper.get_path_genome();
    let cores = calc_similarity(&paths, graph);
    let metric_maxval = cores.iter().max().unwrap();

    let chunksize = (paths.len() + threads - 1) / threads;

    let all_data: Vec<(String, Vec<(u32, u32)>)> = paths
        .par_chunks(chunksize)
        .flat_map(|chunk| {
            let mut local_data = Vec::new();
            for (name, p) in chunk.iter() {
                let mut core_data: Vec<(u32, u32)> = vec![(0, 0); *metric_maxval as usize + 1];
                let mut ll = Vec::new();
                for k in p.iter() {
                    ll.extend(k.nodes.clone());
                }
                ll.sort();
                for x in ll.iter() {
                    let metric_value = cores[*x as usize] as usize;

                    core_data[metric_value].0 += 1;
                    core_data[metric_value].1 += graph.get_sequence_by_id(x).len() as u32;
                }
                local_data.push((name.clone(), core_data))
            }
            local_data
        })
        .collect();
    all_data
}
