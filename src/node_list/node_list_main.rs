use std::process;
use crate::node_list::wrapper::wrapper_node;
use clap::ArgMatches;
use gfa_reader::{check_numeric_gfafile, Gfa, Pansn};
use log::info;

/// Main function for node list
pub fn nodelist_main(matches: &ArgMatches) {
    info!("Running 'gretl node-list'");
    let mut sep = " ";
    if matches.is_present("Pan-SN") {
        sep = matches.value_of("Pan-SN").unwrap();
    }

    if check_numeric_gfafile(matches.value_of("gfa").unwrap()) {
        // Parse GFA file + Wrapper
        let mut graph: Gfa<u32, (), ()> = Gfa::parse_gfa_file(matches.value_of("gfa").unwrap());
        graph.walk_to_path(sep);
        let wrapper: Pansn<u32, (), ()> = Pansn::from_graph(&graph.paths, sep);

        // Other inputs
        let output = matches.value_of("output").unwrap();
        let splits = vec!["Core", "Length", "Depth", "Core", "ND"];
        let mut split_vec = Vec::new();
        if matches.is_present("Features") {
            split_vec = matches.value_of("Features").unwrap().split(',').collect();
        }
        let mut final_features = Vec::new();
        for x in split_vec.iter() {
            if splits.contains(x) {
                final_features.push(*x);
            }
        }

        if final_features.is_empty() {
            final_features = splits.clone();
        }
        info!("Graph file: {}", matches.value_of("gfa").unwrap());
        info!("Output file: {}", output);
        info!("Features: {:?}", final_features);

        info!("Running wrapper + writing direclty to file");
        // This wrapper also writes data to a file
        wrapper_node(&graph, &wrapper, output, final_features);
        info!("Finished writing to file");
    } else {
        eprintln!("Error: GFA file is not numeric");
        process::exit(1);
    }

}
