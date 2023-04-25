use gfa_reader::Gfa;

/// Output
pub fn wrapper(graph: &Gfa) -> Vec<(u32, u32)>{
    let mut result = Vec::new();
    for (id, node) in graph.nodes.iter(){
        result.push((node.len as u32, 1));
    }
    result


}