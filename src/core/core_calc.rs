use gfa_reader::{Gfa, Pansn};

use rayon::prelude::*;

/// Compute the amount of sequence in each similarity level
pub fn pan_genome(
    gwrapper: &Pansn<u32, (), ()>,
    graph: &Gfa<u32, (), ()>,
    stats: &Vec<u32>,
    threads: usize,
) -> (Vec<(usize, usize)>, Vec<(String, usize, usize)>) {
    let paths = gwrapper.get_path_genome();

    // Get additional information for private nodes
    let chunk_size = (paths.len() + threads - 1) / threads;

    let private_only: Vec<(String, usize, usize)> = paths
        .par_chunks(chunk_size)
        .flat_map(|chunk| {
            let mut local = Vec::new();
            for path in chunk.iter() {
                let mut nodes = 0;
                let mut seq = 0;
                for x in path.1.iter() {
                    for node in x.nodes.iter() {
                        let level = stats[*node as usize] as usize;
                        if level == 1 {
                            nodes += 1;
                            seq += graph.get_sequence_by_id(node).len();
                        }
                    }
                }
                local.push((path.0.clone(), nodes, seq));
            }
            local
        })
        .collect();

    // Iterate over the data set (e.g. similarity) and summarize the sequence and nodes for each node
    let max_value = stats.iter().max().unwrap();
    let chunk_size = (stats.len() + threads - 1) / threads;

    let similarity_level: Vec<(usize, usize, usize)> = stats
        .par_chunks(chunk_size)
        .enumerate()
        .flat_map(|(ii, chunk)| {
            let mut local: Vec<(usize, usize, usize)> =
                (0..*max_value as usize + 1).map(|x| (0, 0, x)).collect();
            for (i, x) in chunk.iter().enumerate() {
                if *x != 0 {
                    let i = i + ii * chunk_size;
                    local[*x as usize].0 += 1;
                    local[*x as usize].1 += graph.get_sequence_by_id(&(i as u32)).len();
                }
            }

            local
        })
        .collect();

    let mut result_similarity = vec![(0, 0); *max_value as usize + 1];

    similarity_level.iter().for_each(|x| {
        result_similarity[x.2].0 += x.0;
        result_similarity[x.2].1 += x.1;
    });

    // Check if both values are the same (should be)
    let total_sum: usize = private_only.iter().map(|n| n.2).sum();
    if total_sum == similarity_level.get(1).unwrap().1 {
        eprintln!("Statistic is fine")
    }
    (result_similarity, private_only)
}
