use clap::ArgMatches;
use gfa_reader::{Gfa, GraphWrapper};
use crate::core::writer::writer_core;
use crate::stats::helper::{core2};


/// Core main function
///
/// Calculate amount of nodes and sequence for each level.
/// Everything is written in one file. 
pub fn core_main(_matches: &ArgMatches, graph: &GraphWrapper, graph2: &Gfa, output: &str){
    eprintln!("Running core analysis");
    let cores = core2(graph, graph2);

    // Each entry hold information for its level (#nodes, seq)
    let mut similarity_level: Vec<(usize, usize)> = vec![(0, 0); graph2.paths.len()+1];

    // Get additional information for private nodes
    let mut private_only = Vec::new();
    for path in graph.genomes.iter(){
        let mut nodes = 0;
        let mut seq = 0;
        for x in path.1.iter() {
            for node in x.nodes.iter() {
                let level = cores[&node.parse::<u32>().unwrap()] as usize;
                similarity_level[level].0 += 1;
                similarity_level[level].1 += graph2.nodes.get(node).unwrap().len;
                if level == 1 {
                    nodes += 1;
                    seq += graph2.nodes.get(node).unwrap().len;
                }
            }
        }
        private_only.push((path.0.clone(), nodes, seq));
    }




    // Check if both values are the same (should be)
    let total_sum:usize = private_only.iter().map(|n| n.2).sum();
    if total_sum == similarity_level.get(1).unwrap().1 as usize{
        eprintln!("Statistic is fine")
    }
    // Write output in table
    writer_core(similarity_level, private_only, output)
}

