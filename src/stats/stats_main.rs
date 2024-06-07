use crate::stats::graph_stats::graph_stats_wrapper;

use crate::stats::path_stats::{convert_data, path_stats_wrapper, remove_unsorted};
use crate::stats::stats_writer::{
    write_tsv_graph, write_tsv_path, write_yaml_graph, write_yaml_path,
};
use clap::ArgMatches;
use gfa_reader::{check_numeric_compact_gfafile, check_numeric_gfafile, Gfa, Pansn};
use log::info;

/// Main function for stats subcommand
///
/// This command should return statistics for total graph or path + write everything to a file
pub fn stats_main(matches: &ArgMatches) {
    info!("Running 'gretl stats'");
    let mut sep = " ";
    if matches.is_present("PanSN") {
        sep = matches.value_of("PanSN").unwrap();
        sep = sep.trim();
    }
    let haplo = matches.is_present("haplo");
    let num_com = check_numeric_compact_gfafile(matches.value_of("gfa").unwrap());
    if num_com.0 == true {
        if num_com.1 == false{
            eprintln!("Error: The GFA file is not sorted. All 'jump' stats might be without sense.")
        }
        info!("Reading graph");
        let mut graph: Gfa<u32, (), ()> = Gfa::parse_gfa_file(matches.value_of("gfa").unwrap());
        graph.walk_to_path(sep);

        info!("Creating wrapper");
        let wrapper: Pansn<u32, (), ()> = Pansn::from_graph(&graph.paths, sep);
        let output = matches.value_of("output").unwrap();

        let mut bins: Vec<u32> = vec![1, 50, 100, 1000];
        if matches.is_present("bins") {
            let bins_str = matches.value_of("bins").unwrap();
            bins = bins_str
                .split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
        }

        if matches.is_present("path") {
            info!("Calculating path stats");
            let mut data = path_stats_wrapper(&graph, &wrapper, haplo);
            let mut data = convert_data(&mut data);
            remove_unsorted(&mut data, &graph);

            if matches.is_present("YAML") {
                write_yaml_path(&data, output);
            } else {
                write_tsv_path(&data, output);
            }
        } else {
            info!("Calculating graph stats");
            let data = graph_stats_wrapper(&graph, &wrapper, bins, haplo);

            info!("Writing to file");
            if matches.is_present("YAML") {
                write_tsv_graph(&data, output);
            } else {
                write_yaml_graph(&data, output);
            }
        }
    } else {
        eprintln!("Error: The GFA file is not numeric")
    }
}
