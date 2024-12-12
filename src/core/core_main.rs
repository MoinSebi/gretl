use crate::core::core_calc::pan_genome;
use crate::core::writer::writer_core;
use crate::helpers::graphs::get_stats;
use clap::ArgMatches;
use gfa_reader::{check_numeric_gfafile, Gfa, Pansn};
use log::info;

/// Core main function
///
/// Calculate amount of nodes and sequence for each level.
/// Everything is written in one file.
pub fn core_main(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    // Reading the graph and converting it to a graph wrapper
    info!("Running 'gretl core'");
    let mut sep = matches.value_of("Pan-SN").unwrap();
    let output = matches.value_of("output").unwrap();
    let threads = matches
        .value_of("threads")
        .unwrap()
        .parse::<usize>()
        .expect("Error: Threads must be a number");

    info!("GFA file: {}", matches.value_of("gfa").unwrap());
    info!(
        "Pan-SN: {}",
        if sep == "\n" {
            "None".to_string()
        } else {
            format!("{:?}", sep)
        }
    );
    info!(
        "Output file: {}",
        if output == "-" {
            "stdout".to_string()
        } else {
            output.to_string()
        }
    );
    info!("Threads: {}", threads);
    // Is the graph file numeric?

    info!("Numeric check");
    if check_numeric_gfafile(matches.value_of("gfa").unwrap()) {
        // Check for panSN separator

        info!("Read GFA file");
        let mut graph: Gfa<u32, (), ()> =
            Gfa::parse_gfa_file_multi(matches.value_of("gfa").unwrap(), threads);
        if graph.paths.is_empty() && sep == "\n" {
            sep = "#";
        }
        graph.walk_to_path(sep);

        // Check if paths are found
        if !graph.paths.is_empty() {
            let wrapper: Pansn<u32, (), ()> = Pansn::from_graph(&graph.paths, sep);

            // Get output file name

            // Which kind of stats
            let kind = matches.value_of("statistics").unwrap();
            let stats = get_stats(&wrapper, &graph, kind);

            // Get the data
            // similarity level: amount of nodes for each level
            // private_only: amount of private nodes for accession
            info!("Counting the graph");
            let (similarity_level, private_only) = pan_genome(&wrapper, &graph, &stats, threads);

            // Write output in table
            info!("Writing output");
            writer_core(similarity_level, private_only, output);
            info!("Done");
        } else {
            panic!("Error: No path found in graph file");
        }
    } else {
        panic!("The node IDs in the GFA file are not numeric");    }
    Ok(())
}
