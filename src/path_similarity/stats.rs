use crate::helpers::helper::calc_similarity;
use gfa_reader::{Gfa, Pansn};
use log::info;

/// Compute the amount of sequence in each similarity level
pub fn accession2level(
    graph: &Gfa<u32, (), ()>,
    wrapper: &Pansn<u32, (), ()>,
) -> Vec<(String, Vec<(u32, u32)>)> {
    info!("Run accession to level");
    let paths = wrapper.get_path_genome();
    let cores = calc_similarity(&paths, graph);
    let metric_maxval = cores.iter().max().unwrap();
    let mut res = Vec::new();

    for (name, p) in paths.iter() {
        let mut core_data: Vec<(u32, u32)> = vec![(0, 0); *metric_maxval as usize + 1];
        let mut ll = Vec::new();
        for k in p.iter() {
            ll.extend(k.nodes.clone());
        }
        ll.sort();
        ll.dedup();
        for x in ll.iter() {
            let metric_value = cores[*x as usize] as usize;

            core_data[metric_value].0 += 1;
            core_data[metric_value].1 += graph.get_sequence_by_id(x).len() as u32;
        }
        res.push((name.clone(), core_data));
    }

    res
}
