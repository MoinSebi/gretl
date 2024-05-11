use crate::core::core_calc::pan_genome;
use crate::core::writer::writer_core;
use crate::helpers::graphs::get_stats;
use clap::ArgMatches;
use gfa_reader::{Gfa, Pansn};

/// Core main function
///
/// Calculate amount of nodes and sequence for each level.
/// Everything is written in one file.
pub fn core_main(matches: &ArgMatches) {
    // Reading the graph and converting it to a graph wrapper
    eprintln!("Running core analysis");

    // Check for panSN separator
    let mut sep = " ";
    if matches.is_present("Pan-SN") {
        sep = matches.value_of("Pan-SN").unwrap();
    }
    let mut graph: Gfa<u32, (), ()> = Gfa::parse_gfa_file(matches.value_of("gfa").unwrap());
    println!("Walking to path {}", graph.paths.len());
    graph.walk_to_path(sep);
    let wrapper: Pansn<u32, (), ()> = Pansn::from_graph(&graph.paths, sep);

    // Get output file name
    let output = matches.value_of("output").unwrap();

    // Which kind of stats
    let kind = matches.value_of("statistics").unwrap();
    let stats = get_stats(&wrapper, &graph, kind);

    // Get the data
    // similarity level: amount of nodes for each level
    // private_only: amount of private nodes for accession
    let (similarity_level, private_only) = pan_genome(&wrapper, &graph, &stats);

    // Write output in table
    writer_core(similarity_level, private_only, output)
}
