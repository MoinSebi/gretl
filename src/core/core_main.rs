use clap::ArgMatches;
use gfa_reader::{Gfa, GraphWrapper};
use crate::core::core_calc::core_cal;
use crate::core::writer::writer_core;
use crate::stats::helper::{core2};


/// Core main function
///
/// Calculate amount of nodes and sequence for each level.
/// Everything is written in one file. 
pub fn core_main(_matches: &ArgMatches, graph: &GraphWrapper, graph2: &Gfa, output: &str){
    eprintln!("Running core analysis");
    let (similarity_level, private_only) = core_cal(graph, graph2);
    // Write output in table
    writer_core(similarity_level, private_only, output)
}

