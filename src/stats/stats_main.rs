use clap::ArgMatches;
use gfa_reader::{Gfa, GraphWrapper, NCEdge, NCGfa, NCPath};
use crate::stats::graph_stats::graph_stats_wrapper;
use crate::stats::helper::get_filename;
use crate::stats::path_stats::path_stats_wrapper;
use crate::stats::stats_writer::{write_graph_tsv, write_tsv_path, write_graph_yaml, write_yaml_path};


/// Main function for stats subcommand
///
/// This command should return statistics for total graph or the path()
pub fn stats_main(matches: &ArgMatches){
    let mut graph: NCGfa<()> = NCGfa::new();
    graph.parse_gfa_file_and_convert(matches.value_of("gfa").unwrap(), true);
    let mut wrapper: GraphWrapper<NCPath> = GraphWrapper::new();
    wrapper.from_gfa(&graph.paths, " ");
    let output = matches.value_of("output").unwrap();

    let mut bins = vec![1, 50, 100, 1000];
    if matches.is_present("bins"){
        let bins_str = matches.value_of("bins").unwrap();
        bins = bins_str.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
    }


    if matches.is_present("path"){
        let data = path_stats_wrapper(&graph, &wrapper);

        if matches.is_present("YAML"){
            write_yaml_path(&data, output);
        } else {
            write_tsv_path(&data, output);
        }
    } else {
        let data = graph_stats_wrapper(&graph, &wrapper, bins);
        if matches.is_present("YAML"){
            write_graph_yaml(&data, output);
        } else {
            write_graph_tsv(&data, output);
        }
    }
}