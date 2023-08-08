use clap::ArgMatches;
use gfa_reader::{Edge, Gfa, GraphWrapper, NCGfa, NCPath};
use crate::core::core_calc::core_cal;
use crate::core::writer::writer_core;
use crate::stats::helper::{get_filename};


/// Core main function
///
/// Calculate amount of nodes and sequence for each level.
/// Everything is written in one file. 
pub fn core_main(matches: &ArgMatches){
    eprintln!("Running core analysis");
    let mut graph: NCGfa<()> = NCGfa::new();
    println!("{}", matches.value_of("gfa").unwrap());
    graph.parse_gfa_file_and_convert(matches.value_of("gfa").unwrap(), true);
    let mut wrapper: GraphWrapper<NCPath> = GraphWrapper::new();
    wrapper.from_gfa(&graph.paths, " ");

    let filename = get_filename(matches.value_of("gfa").unwrap());
    let output = matches.value_of("output").unwrap();

    let (similarity_level, private_only) = core_cal(&wrapper, &graph);
    // Write output in table
    writer_core(similarity_level, private_only, output)
}

