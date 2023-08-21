use clap::ArgMatches;
use gfa_reader::{Gfa, GraphWrapper, NCEdge, NCGfa, NCPath};
use crate::stats::graph_stats::graph_stats_wrapper;
use crate::stats::helper::get_filename;
use crate::stats::path_stats::path_stats_wrapper;
use crate::stats::stats_writer::{write_tsv, write_tsv_path, write_yaml, write_yaml_path};


/// Main function for stats subcommand
pub fn stats_main(matches: &ArgMatches){
    let mut graph: NCGfa<()> = NCGfa::new();
    graph.parse_gfa_file_and_convert(matches.value_of("gfa").unwrap(), true);
    let mut wrapper: GraphWrapper<NCPath> = GraphWrapper::new();
    wrapper.from_gfa(&graph.paths, " ");
    let output = matches.value_of("output").unwrap();

    if matches.is_present("path"){
        let data = path_stats_wrapper(&graph, &wrapper);
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
            "Graph_degree_[total]",
            "Inverted_edges",
            "Negative_edges",
            "Self_edges",
            "#Nodes_[size=1]",
            "#Nodes_[size=2]",
            "#Nodes_[size=3]",
            "#Nodes_[size=4]",
            "#Seq[size=1]",
            "#Seq[size=2]",
            "#Seq[size=3]",
            "#Seq[size=4]",

            "Pathseqlen",
            "PathNode",
            "Path unique",
            "Path inverted node",
            "Path inverted seq",
            "Path jumps total",
            "Path jumps bigger",
            "Graph density",

        ];
        if matches.is_present("YAML"){
            write_tsv(&data, &tab, output);
            write_yaml(&data, &tab, output);
        } else {
            write_tsv(&data, &tab, output);
        }
    }
}