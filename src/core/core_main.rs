use std::os::unix::raw::uid_t;
use clap::ArgMatches;
use gfa_reader::Gfa;
use crate::core::writer::writer_core;
use crate::stats::helper::{calculate_core, core1};


/// Core main function
///
/// Check all core levels first
/// -Return amount of each core level + the amount of private for each accession
pub fn core_main(matches: &ArgMatches, graph: &Gfa, output: &str){
    eprintln!("Running core analysis");
    let amount_path = graph.paths.len();
    let cores = core1(graph);
    let mut similarity_level: Vec<(usize, usize)> = vec![(0, 0); graph.paths.len()+1];
    let mut private_only = Vec::new();
    for path in graph.paths.iter(){
        let mut nodes = 0;
        let mut seq = 0;
        for node in path.nodes.iter(){
            let level = cores[&node.parse::<u32>().unwrap()] as usize;
            similarity_level[level].0 += 1;
            similarity_level[level].1 += graph.nodes.get(node).unwrap().len;
            if level == 1{
                nodes += 1;
                seq += graph.nodes.get(node).unwrap().len;
            }

        }
        private_only.push((path.name.clone(), nodes, seq));
    }




    let total_sum:usize = private_only.iter().map(|n| n.2).sum();
    if total_sum == similarity_level.get(1).unwrap().1 as usize{
        eprintln!("Statistic is fine")
    }
    // Write output in table
    writer_core(similarity_level, private_only, output)



    
    // Make a meta file


}

