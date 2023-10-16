use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::vec;
use gfa_reader::{Gfa, GraphWrapper, NCGfa, NCPath};
use crate::node_list::writer::{make_buffer, write_header, write_list};

use std::io::{BufWriter, Write};
use std::fs::File;
use crate::helpers::helper::{calculate_similarity, calculate_depth, node_degree, node_len};

/// Wrapper function for node list analysis
///
pub fn wrapper_node(graph: &NCGfa<()>, wrapper: &GraphWrapper<NCPath>, filename: &str, what: Vec<&str>){


    let mut ff = make_buffer(filename);
    let real_node_name = &graph.mapper;

    write_header(&real_node_name, &mut ff);
    if what.contains(&"Length"){
        let len = node_len(graph);
        write_list( ("Length", &len), &mut ff);
    } if what.contains(&"Core"){
        let core = calculate_similarity(wrapper, graph);
        write_list(("Core", &core), &mut ff);
    } if what.contains(&"Depth"){
        let depth2 = calculate_depth(wrapper, graph);
        write_list(("Depth", &depth2), &mut ff);
    } if what.contains(&"ND"){
        let (nd_out, node_in, node_total) = node_degree(graph);
        write_list(("ND_out", &nd_out), &mut ff);
        write_list(("ND_in", &node_in), &mut ff);
        write_list(("ND_in", &node_total), &mut ff);
    }
}




