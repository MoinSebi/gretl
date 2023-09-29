use std::collections::HashSet;
use gfa_reader::{GraphWrapper, NCGfa, NCPath};
use crate::helpers::helper::{calculate_core, calculate_depth, node_degree, node_degree_total};

pub fn get_stats(wrapper: &GraphWrapper<NCPath>, graph: &NCGfa<()>, kind: &str) -> Vec<u32>{
    if kind == "depth" {
        return calculate_depth(wrapper, graph);
    } else if kind == "nd" {
        return node_degree_total(graph);
    } else {
        return calculate_core(wrapper, graph);
    }
}
