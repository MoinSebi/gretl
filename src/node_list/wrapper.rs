use crate::helpers::helper::{calc_depth, calc_node_degree, calc_node_len, calc_similarity};
use crate::node_list::writer::{make_buffer, write_header, write_list};
use gfa_reader::{Gfa, Pansn};
use log::info;

/// Wrapper function for node list analysis
///
pub fn wrapper_node(
    graph: &Gfa<u32, (), ()>,
    wrapper: &Pansn<u32, (), ()>,
    filename: &str,
    what: Vec<&str>,
) {
    let paths = wrapper.get_path_genome();

    let mut ff = make_buffer(filename);
    let len = calc_node_len(graph);
    let po = get_zero_vec(&len);
    write_header(&len, &mut ff);
    //write_header(real_node_name, &mut ff);
    if what.contains(&"Length") {
        write_list(("Length", &len), &mut ff, &po);
    }
    if what.contains(&"Core") {
        let core = calc_similarity(&paths, graph);
        write_list(("Core", &core), &mut ff, &po);
    }
    if what.contains(&"Depth") {
        let depth2 = calc_depth(&paths, graph);
        write_list(("Depth", &depth2), &mut ff, &po);
    }
    if what.contains(&"ND") {
        let (nd_out, node_in, node_total) = calc_node_degree(graph);
        write_list(("ND_out", &nd_out), &mut ff, &po);
        write_list(("ND_in", &node_in), &mut ff, &po);
        write_list(("ND_in", &node_total), &mut ff, &po);
    }
}

pub fn get_zero_vec(size: &Vec<u32>) -> Vec<bool> {
    let mut o = Vec::new();
    for k in size.iter() {
        if k != &0 {
            o.push(true);
        } else {
            o.push(false);
        }
    }
    o
}
