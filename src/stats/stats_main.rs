use crate::stats::graph_stats::graph_stats_wrapper;

use crate::stats::path_stats::{convert_data, path_stats_wrapper, remove_unsorted};
use crate::stats::stats_writer::{
    write_tsv_graph, write_tsv_path, write_yaml_graph, write_yaml_path,
};
use clap::ArgMatches;
use gfa_reader::{NCGfa, NCPath, Pansn};

/// Main function for stats subcommand
///
/// This command should return statistics for total graph or path + write everything to a file
pub fn stats_main(matches: &ArgMatches) {
    let mut sep = " ";
    if matches.is_present("Pan-SN") {
        sep = matches.value_of("Pan-SN").unwrap();
        sep = sep.trim();
    }
    let _haplo = matches.is_present("haplo");

    let mut graph: NCGfa<()> = NCGfa::new();
    println!("{}", matches.value_of("gfa").unwrap());
    graph.parse_gfa_file_and_convert(matches.value_of("gfa").unwrap(), true);
    let wrapper: Pansn<NCPath> = Pansn::from_graph(&graph.paths, sep);
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
        let mut data = path_stats_wrapper(&graph, &wrapper, matches.is_present("haplo"));
        let mut data = convert_data(&mut data);
        remove_unsorted(&mut data, &graph);

        if matches.is_present("YAML") {
            write_yaml_path(&data, output);
        } else {
            write_tsv_path(&data, output);
        }
    } else {
        let data = graph_stats_wrapper(&graph, &wrapper, bins, matches.is_present("haplo"));
        if matches.is_present("YAML") {
            write_tsv_graph(&data, output);
        } else {
            write_yaml_graph(&data, output);
        }
    }
}
