use crate::id2int::convert_graph::*;
use clap::ArgMatches;
use gfa_reader::{Gfa, NGfa};
use crate::helpers::graphs::{make_wrapper, read_graph};
use crate::id2int::writer::writer_graph;
use crate::stats::helper::get_filename;


/// Main function for node id to integer function
pub fn id2int_main(matches: &ArgMatches){
    let graph = read_graph(matches);
    let gw = make_wrapper(&graph, matches);
    let output = matches.value_of("output").unwrap();
    eprintln!("ID2INT");
    eprintln!("Convert non digit node identifiers to numeric and compress ID space (not smart)");

    let (id2int, new_nodes) = id2int_nnodes(&graph);
    let new_edges = nedges(&graph, &id2int);
    let (new_paths, path2id) = path_new(&graph, &id2int);
    let new_graph = NGfa{nodes: new_nodes, paths: new_paths, edges: new_edges, path2id: path2id};

    writer_graph(new_graph, output);
}

