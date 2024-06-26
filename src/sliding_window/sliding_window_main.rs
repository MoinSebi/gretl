use crate::sliding_window::window::sliding_window_wrapper;
use crate::sliding_window::writer::write_window;
use clap::ArgMatches;
use gfa_reader::{check_numeric_gfafile, Gfa, Pansn};
use log::info;

/// Main function for node id to integer function
pub fn window_main(matches: &ArgMatches) {
    info!("Running 'gretl window'");

    if check_numeric_gfafile(matches.value_of("gfa").unwrap()) {
        let mut graph: Gfa<u32, (), ()> = Gfa::parse_gfa_file(matches.value_of("gfa").unwrap());
        graph.walk_to_path("#");
        if graph.paths.is_empty() {
            panic!("Error: No path found in graph file")
        }
        let wrapper: Pansn<u32, (), ()> = Pansn::from_graph(&graph.paths, " ");

        let output = matches.value_of("output").unwrap();
        let mut size: u32 = 100000;
        if matches.is_present("window-size") {
            size = matches.value_of("window-size").unwrap().parse().unwrap();
        }

        let mut step: u32 = size;
        if matches.is_present("moving-size") {
            step = matches.value_of("moving-size").unwrap().parse().unwrap();
        }

        let mut node = false;
        if matches.is_present("node") {
            node = true;
        }

        // similarity
        let mut metric = Metric::Similarity;
        if matches.is_present("metric") {
            match matches.value_of("metric").unwrap() {
                "similarity" => metric = Metric::Nodesizem,
                "nodesize" => metric = Metric::Similarity,
                "depth" => metric = Metric::Depth,
                _ => metric = Metric::Similarity,
            }
        }
        info!("Gfa file: {}", matches.value_of("gfa").unwrap());
        info!("Output file: {}", output);
        info!("Window size: {}", size);
        info!("Moving size: {}", step);
        info!("Node: {}", node);
        info!("Metric: {:?}", "Similarity");

        info!("Sliding window analysis");
        let f = sliding_window_wrapper(&graph, &wrapper, size, step, metric, node);
        info!("Writing to file");
        write_window(f, output);
    } else {
        panic!("Error: GFA file is not numeric");
    }
}

pub enum Metric {
    Similarity,
    Nodesizem,
    Depth,
}
