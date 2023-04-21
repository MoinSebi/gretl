use clap::ArgMatches;
use gfa_reader::Gfa;
use crate::stats::graph_stats::graph_stats_wrapper;
use crate::stats::path::path_stats_wrapper;
use crate::stats::writer::{write_tsv_path, write_yaml, write_yaml_path};

pub fn stats_main(matches: &ArgMatches, graph: &Gfa, output: &str){
    if matches.is_present("path"){
        let data = path_stats_wrapper(&graph);
        let tab = [
            "Node length (seq)",
            "Nodes length (node)",
            "Unique nodes",
            "Inverted nodes",
            "Inverted nodes",
            "Jumps total",
            "Jumps ratio",
            "Jumps bigger than ",
            "Average depth",
            "Median depth",
            "Average similarity",
            "Median similarity",
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
            "Node length [average]",
            "Node length [mediant]",
            "Node length [total]",
            "Input genomes [total]",
            "Graph degree [in]",
            "Graph degree [out]",
            "Graph degree [total]"];
        write_yaml(&data, &tab, output);
    }
}