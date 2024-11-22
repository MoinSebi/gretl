use crate::path_similarity::ps_stats::accession2level;
use crate::path_similarity::writer_test::write_ps;
use clap::ArgMatches;
use gfa_reader::{check_numeric_gfafile, Gfa, Pansn};
use log::{info, warn};

/// Main function for path related stats
pub fn ps_main(matches: &ArgMatches) {
    let threads = matches
        .value_of("threads")
        .unwrap()
        .parse::<usize>()
        .expect("Error: Threads must be a number");

    let mut pansn = matches.value_of("PanSN").unwrap();
    let output = matches.value_of("output").unwrap();

    info!("Running 'gretl ps' analysis");
    info!("GFA file: {}", matches.value_of("gfa").unwrap());
    info!(
        "Pan-SN: {}",
        if pansn == "\n" {
            "None".to_string()
        } else {
            format!("{:?}", pansn)
        }
    );
    info!("Threads: {}", threads);
    info!(
        "Output file: {}",
        if output == "-" {
            "stdout".to_string()
        } else {
            output.to_string()
        }
    );

    if pansn == "\n" {
        let message =
            r"No PanSN provided, using default PanSN '\n'. Every path will be its own sample!";
        warn!("{}", message);
    }

    // Check numeric
    info!("Numeric check");
    if check_numeric_gfafile(matches.value_of("gfa").unwrap()) {
        // Read the graph

        info!("Read GFA file");

        let mut graph: Gfa<u32, (), ()> =
            Gfa::parse_gfa_file_multi(matches.value_of("gfa").unwrap(), threads);

        if graph.paths.is_empty() && pansn == "\n" {
            pansn = "#";
        }
        // Convert walk to path
        graph.walk_to_path(pansn);
        if graph.paths.is_empty() {
            panic!("Error: No path found in graph file")
        }

        // Check if paths are found (path are needed for this analysis)
        let wrapper: Pansn<u32, (), ()> = Pansn::from_graph(&graph.paths, pansn);
        info!("Accession2level");
        let data = accession2level(&graph, &wrapper, threads);

        info!("Writing path similarity data to file");
        write_ps(&data, output);

        info!("Done");
    } else {
        panic!("Error: GFA file is not numeric");
    }
}
