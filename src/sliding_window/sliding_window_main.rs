use crate::sliding_window::window::sliding_window_wrapper;
use crate::sliding_window::writer::write_window;
use clap::ArgMatches;
use gfa_reader::{check_numeric_gfafile, Gfa, Pansn};
use log::{info, warn};

/// Main function for node id to integer function
pub fn window_main(matches: &ArgMatches) {
    info!("Running 'gretl window'");

    let _graph = matches.value_of("gfa").unwrap();
    let mut pansn = matches.value_of("Pan-SN").unwrap();
    let output = matches.value_of("output").unwrap();
    let window_size = matches
        .value_of("window-size")
        .unwrap_or("100000")
        .parse()
        .unwrap();
    let step_size = matches
        .value_of("step-size")
        .unwrap_or("100000")
        .parse()
        .unwrap();
    let node = matches.is_present("node");
    let threads = matches
        .value_of("threads")
        .unwrap()
        .parse()
        .expect("Error: Threads must be a number");

    let mut metric = Metric::Similarity;
    if matches.is_present("metric") {
        match matches.value_of("metric").unwrap() {
            "similarity" => metric = Metric::Similarity,
            "nodesize" => metric = Metric::Nodesizem,
            "depth" => metric = Metric::Depth,
            _ => metric = Metric::Similarity,
        }
    }

    info!("GFA file: {}", matches.value_of("gfa").unwrap());
    info!("Window size: {}", window_size);
    info!("Step size: {}", step_size);
    info!("Node: {}", node);
    info!("Metric: {:?}\n", metric.to_string());
    info!(
        "Output file: {}",
        if output == "-" { "stdout" } else { output }
    );

    info!("Numeric check");
    if check_numeric_gfafile(matches.value_of("gfa").unwrap()) {
        info!("Read GFA file");
        let mut graph: Gfa<u32, (), ()> =
            Gfa::parse_gfa_file_multi(matches.value_of("gfa").unwrap(), threads);

        if graph.paths.is_empty() && pansn == "\n" {
            pansn = "#";
        }
        graph.walk_to_path(pansn);

        if graph.paths.is_empty() {
            panic!("Error: No path found in graph file")
        }
        let wrapper: Pansn<u32, (), ()> = Pansn::from_graph(&graph.paths, "\n");

        if !matches.is_present("window-size") && !matches.is_present("step-size") {
            warn!("Running on default values for window site and step size. Window-size: 100000 and step-size: 100000");
        }

        info!("Sliding window analysis");
        let f = sliding_window_wrapper(
            &graph,
            &wrapper,
            window_size,
            step_size,
            metric,
            node,
            threads,
        );
        info!("Writing to file");
        write_window(f, output, node, window_size, step_size);
        info!("Done")
    } else {
        panic!("The node IDs in the GFA file are not numeric");
    }
}

pub enum Metric {
    Similarity,
    Nodesizem,
    Depth,
}

impl Metric {
    pub fn to_string(&self) -> String {
        match self {
            Metric::Similarity => "Similarity".to_string(),
            Metric::Nodesizem => "Node size".to_string(),
            Metric::Depth => "Depth".to_string(),
        }
    }
}
