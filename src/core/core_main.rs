use std::os::unix::raw::uid_t;
use clap::ArgMatches;
use gfa_reader::Gfa;
use crate::stats::helper::calculate_core;


/// Core main function
///
/// Check all core levels first
/// -Return amount of each core level + the amount of private for each accession
pub fn core_main(matches: &ArgMatches, graph: &Gfa){
    eprintln!("Running core analysis");
    let amount_path = graph.paths.len();
    let cores = calculate_core(graph);

    let mut total22 = vec![];
    for p in graph.paths.iter(){
        let mut total = 0;
        let mut total2 = 0;
        for x in p.nodes.iter(){
            if cores[&x.parse::<u32>().unwrap()].0 == 1{
                total += 1;
                total2 += graph.nodes.get(x).unwrap().len;
            }

        }
        total22.push((total, total2));
    }


    
    // Make a meta file


}

