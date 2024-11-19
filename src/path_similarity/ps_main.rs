use crate::path_similarity::ps_stats::accession2level;
use crate::path_similarity::writer_test::write_ps;
use clap::ArgMatches;
use gfa_reader::{check_numeric_gfafile, Gfa, Pansn};
use log::{info, warn};

/// Main function for path related stats
pub fn ps_main(matches: &ArgMatches) {

    // Check numeric
    if check_numeric_gfafile(matches.value_of("gfa").unwrap()) {
        // Read the graph
        let threads = matches
            .value_of("threads")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let mut pansn = matches.value_of("PanSN").unwrap();
        let output = matches.value_of("output").unwrap();

        info!("Running 'gretl ps' analysis");
        info!("Graph file: {}", matches.value_of("gfa").unwrap());
        info!("Separator: {}", if pansn == "\n" { "None".to_string() } else { format!("{:?}", pansn) });
        info!("Threads: {}", threads);
        info!("Output file: {}", if output == "-" { "stdout".to_string() } else { output.to_string() });

        if pansn == "\n"{
            let message = r"No PanSN provided, using default PanSN '\n'. Every path will be its own sample!";
            warn!("{}" ,message);
        }

        info!("Reading graph file");

            let mut graph: Gfa<u32, (), ()> = Gfa::parse_gfa_file_multi(matches.value_of("gfa").unwrap(), threads);

            if graph.paths.is_empty() && pansn == "\n" {
                pansn = "#";

            }
            // Convert walk to path
            graph.walk_to_path(pansn);

            // Check if paths are found (path are needed for this analysis)
            let wrapper: Pansn<u32, (), ()> = Pansn::from_graph(&graph.paths, pansn);
            let data = accession2level(&graph, &wrapper, threads);

            write_ps(&data, output);
        } else {
            panic!("Error: GFA file is not numeric");
        }

}
