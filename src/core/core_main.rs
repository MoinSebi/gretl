use clap::ArgMatches;
use gfa_reader::{Gfa, GraphWrapper};
use crate::core::core_calc::core_cal;
use crate::core::writer::writer_core;
use crate::helpers::graphs::{make_wrapper, read_graph};
use crate::stats::helper::{core2, get_filename};


/// Core main function
///
/// Calculate amount of nodes and sequence for each level.
/// Everything is written in one file. 
pub fn core_main(matches: &ArgMatches){
    eprintln!("Running core analysis");
    let graph = read_graph(matches);
    println!("{}", graph.nodes.len());
    let gw = make_wrapper(&graph, matches);
    let filename = get_filename(matches.value_of("gfa").unwrap());
    let output = matches.value_of("output").unwrap();

    let (similarity_level, private_only) = core_cal(&gw, &graph);
    // Write output in table
    writer_core(similarity_level, private_only, output)
}

