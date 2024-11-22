use crate::stats::graph_stats::graph_stats_wrapper;

use crate::stats::path_stats::{convert_data, path_stats_wrapper, remove_unsorted};
use crate::stats::stats_writer::{
    write_tsv_graph, write_tsv_path, write_yaml_graph, write_yaml_path,
};
use clap::ArgMatches;
use gfa_reader::{check_numeric_compact_gfafile, Gfa, Pansn};
use log::{info, warn};

/// Main function for stats subcommand
///
/// This command should return statistics for total graph or path + write everything to a file
pub fn stats_main(matches: &ArgMatches) {
    info!("Running 'gretl stats'");
    let mut sep = matches.value_of("PanSN").unwrap();
    let haplo = matches.is_present("haplo");
    let gfafile = matches.value_of("gfa").unwrap();
    let mut bins: Vec<u32> = vec![1, 50, 100, 1000];
    if matches.is_present("bins") {
        let bins_str = matches.value_of("bins").unwrap();
        bins = bins_str
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
    }
    let want_path = matches.is_present("path");
    let output = matches.value_of("output").unwrap();
    let threads = matches.value_of("threads").unwrap().parse().unwrap();

    info!("Gfa file: {}", gfafile);
    info!(
        "PanSN separator: {:?}",
        if sep == "\n" { "None" } else { sep }
    );
    info!("Bins: {:?}", bins);
    info!("Haplo: {}", haplo);
    info!("Path statistics: {}", want_path);
    info!("Threads: {}", threads);
    info!(
        "Output format: {}",
        if matches.is_present("YAML") {
            "YAML"
        } else {
            "TSV"
        }
    );
    info!(
        "Output file: {}\n",
        if output == "-" { "stdout" } else { output }
    );

    info!("Numeric check");
    let num_com = check_numeric_compact_gfafile(matches.value_of("gfa").unwrap());

    if num_com.0 {
        if !num_com.1 {
            warn!("The GFA file is not sorted. All 'jump' stats might be without sense.")
        }
        info!("Read GFA file");
        let mut graph: Gfa<u32, (), ()> = Gfa::parse_gfa_file_multi(gfafile, threads);
        if graph.paths.is_empty() && sep == "\n" {
            sep = "#";
        }
        graph.walk_to_path(sep);

        if graph.paths.is_empty() {
            panic!("Error: No path found in graph file")
        }

        info!("Creating wrapper");
        let wrapper: Pansn<u32, (), ()> = Pansn::from_graph(&graph.paths, sep);

        if want_path {
            info!("Calculating path stats");
            let mut data = path_stats_wrapper(&graph, &wrapper, haplo, threads);
            let mut data = convert_data(&mut data);
            remove_unsorted(&mut data, &graph);

            if matches.is_present("YAML") {
                write_yaml_path(&data, output);
            } else {
                write_tsv_path(&data, output);
            }
        } else {
            info!("Calculating graph stats");
            let data = graph_stats_wrapper(&graph, &wrapper, bins, haplo, threads);

            info!("Writing to file");
            if matches.is_present("YAML") {
                write_tsv_graph(&data, output);
            } else {
                write_yaml_graph(&data, output);
            }
        }
        info!("Done");
    } else {
        panic!("Error: The GFA file is not numeric")
    }
}
