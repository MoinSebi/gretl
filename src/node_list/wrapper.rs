use crate::helpers::helper::{calculate_depth, calculate_similarity, node_degree, node_len};
use crate::node_list::writer::{make_buffer, write_header, write_list};
use gfa_reader::{Gfa, Pansn};

/// Wrapper function for node list analysis
///
pub fn wrapper_node(graph: &Gfa<u32, (), ()>, wrapper: &Pansn<u32, (), ()>, filename: &str, what: Vec<&str>) {
    let paths = wrapper.get_path_genome();

    let mut ff = make_buffer(filename);

    //write_header(real_node_name, &mut ff);
    if what.contains(&"Length") {
        let len = node_len(graph);
        write_list(("Length", &len), &mut ff);
    }
    if what.contains(&"Core") {
        let core = calculate_similarity(&paths, graph);
        write_list(("Core", &core), &mut ff);
    }
    if what.contains(&"Depth") {
        let depth2 = calculate_depth(&paths, graph);
        write_list(("Depth", &depth2), &mut ff);
    }
    if what.contains(&"ND") {
        let (nd_out, node_in, node_total) = node_degree(graph);
        write_list(("ND_out", &nd_out), &mut ff);
        write_list(("ND_in", &node_in), &mut ff);
        write_list(("ND_in", &node_total), &mut ff);
    }
}
