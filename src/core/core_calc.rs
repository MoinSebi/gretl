use gfa_reader::{Gfa, GraphWrapper, NCGfa, NCPath, Path};
use crate::helpers::helper::calculate_core;

pub fn core_cal(gwrapper: &GraphWrapper<NCPath>, graph: &NCGfa<()>) -> (Vec<(usize, usize)>, Vec<(String, usize, usize)>){
    eprintln!("Running core analysis");
    let cores = calculate_core(gwrapper, graph);
    // Each entry hold information for its level (#nodes, seq)

    // Get additional information for private nodes
    let mut private_only: Vec<(String, usize, usize)> = Vec::new();


    for path in gwrapper.genomes.iter(){
        let mut nodes = 0;
        let mut seq = 0;
        for x in path.1.iter() {
            for node in x.nodes.iter() {
                let level = cores[*node as usize - 1].clone() as usize;
                if level == 1 {
                    nodes += 1;
                    seq += graph.nodes[*node as usize -1].seq.len();
                }
            }
        }
        private_only.push((path.0.clone(), nodes, seq));
    }

    let mut similarity_level: Vec<(usize, usize)> = vec![(0, 0); graph.paths.len()+1];
    for (i, x) in cores.iter().enumerate() {
        similarity_level[*x as usize].0 += 1;
        similarity_level[*x as usize].1 += graph.nodes[i].seq.len();
    }




    // Check if both values are the same (should be)
    let total_sum: usize = private_only.iter().map(|n| n.2).sum();
    if total_sum == similarity_level.get(1).unwrap().1.clone() as usize{
        eprintln!("Statistic is fine")
    }
    return (similarity_level, private_only)
}