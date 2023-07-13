use clap::ArgMatches;
use gfa_reader::{Gfa, NCGraphWrapper};
use crate::helpers::graphs::{make_wrapper, read_graph};
use crate::stats::graph_stats::graph_stats_wrapper;
use crate::stats::helper::get_filename;
use crate::stats::path_stats::path_stats_wrapper;
use crate::stats::stats_writer::{write_tsv, write_tsv_path, write_yaml, write_yaml_path};


/// Main function for stats subcommand
pub fn stats_main(matches: &ArgMatches){
    let graph = read_graph(matches);
    let mut nc_wrapper = NCGraphWrapper::new();
    nc_wrapper.from_ngfa(&graph, "");
    let output = matches.value_of("output").unwrap();

    if matches.is_present("path"){
        let data = path_stats_wrapper(&graph, &nc_wrapper);
        let tab = [
            "Node_length_(seq)",
            "Nodes_length_(node)",
            "Unique_nodes",
            "Inverted_nodes",
            "Inverted_nodes",
            "Jumps_total",
            "Jumps_ratio",
            "Jumps_bigger than ",
            "Average_depth",
            "Median_depth",
            "Average_similarity",
            "Median_similarity",
        "Degree"];
        if matches.is_present("YAML"){
            write_yaml_path(&data, &tab, output);
        } else {
            write_tsv_path(&data, &tab, output);
        }
    } else {
        let data = graph_stats_wrapper(&graph);
        let tab = ["#Path",
            "#Nodes",
            "#Edges",
            "Node_length_[average]",
            "Node_length_[median]",
            "Node_length_[total]",
            "Input_genomes_[total]",
            "Graph_degree_[in]",
            "Graph_degree_[out]",
            "Graph_degree_[total]"];
        if matches.is_present("YAML"){
            write_tsv(&data, &tab, output);
            write_yaml(&data, &tab, output);
        } else {
            write_tsv(&data, &tab, output);
        }
    }
}