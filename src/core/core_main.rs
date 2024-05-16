use std::process;
use crate::core::core_calc::pan_genome;
use crate::core::writer::writer_core;
use crate::helpers::graphs::get_stats;
use clap::ArgMatches;
use gfa_reader::{check_numeric_gfafile, Gfa, Pansn};

/// Core main function
///
/// Calculate amount of nodes and sequence for each level.
/// Everything is written in one file.
pub fn core_main(matches: &ArgMatches) {
    // Reading the graph and converting it to a graph wrapper
    eprintln!("Running 'gretl core' analysis");
    // Is the graph file numeric?
    if check_numeric_gfafile(matches.value_of("gfa").unwrap()) {

        // Check for panSN separator
        let mut sep = " ";
        if matches.is_present("Pan-SN") {
            sep = matches.value_of("Pan-SN").unwrap();
        }

        // Read graph and parser
        let mut graph: Gfa<u32, (), ()> = Gfa::parse_gfa_file(matches.value_of("gfa").unwrap());
        println!("Walking to path {}", graph.paths.len());
        graph.walk_to_path(sep);

        // Check if paths are found
        if graph.paths.len() != 0 {
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
        else {
            eprintln!("Error: No path found in graph file");
            process::exit(1)
        }

    } else {
        eprintln!("Error: Graph file is not numeric");
        process::exit(1)
    }
}
