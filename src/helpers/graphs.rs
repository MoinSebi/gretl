use crate::helpers::helper::{calc_depth, calc_node_degree, calc_similarity};
use gfa_reader::{Gfa, Pansn, Segment, SeqIndex};

pub fn get_stats(wrapper: &Pansn<u32, (), ()>, graph: &Gfa<u32, (), ()>, kind: &str) -> Vec<u32> {
    let paths = wrapper.get_path_genome();

    if kind == "depth" {
        calc_depth(&paths, graph)
    } else if kind == "nd" {
        return calc_node_degree(graph).2;
    } else {
        return calc_similarity(&paths, graph);
    }
}

