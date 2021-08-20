use gfaR::Gfa;
use std::collections::{HashMap};

/// Compute mean+median node size and total graph size
///
/// Sum all nodes in the graph and divide it by the number of nodes
///
///
pub fn mean_median_graph_size(graph: &Gfa) -> Vec<(&str, String)> {
    let mut numb: u32 = 0;
    let mut total_size: u32 = 0;
    let mut vec_size: Vec<u32> = Vec::new();
    for x in &graph.nodes{
        numb += 1;
        total_size += x.1.len as u32;
        vec_size.push(x.1.len as u32);

    }
    vec_size.sort();
    let mid = vec_size.len()/2;
    let median = vec_size[mid];
    let mean: f32 = (total_size as f32)/(numb as f32);

    let mut result: Vec<(&str, String)> = Vec::new();
    result.push(("Node mean size [bp]", format!("{:.4}", mean)));
    result.push(("Node median size [bp]", median.to_string()));
    result.push(("Graph size  [bp]", total_size.to_string()));

    result
}

/// Calculate total size of all input genomes
///
/// Iterate over path information - sum length of all nodes
///
pub fn input_genomes(graph: &Gfa) -> Vec<(&str, String)> {
    let mut input_size: u32 = 0;

    for x in &graph.paths{
        for y in &x.nodes{
            input_size += graph.nodes[y].len as u32;
        }
    }

    let mut result:Vec<(&str, String)> = Vec::new();
    result.push(("Input genome [bp]", input_size.to_string()));
    result
}

/// Number of in and outgoing edges + total
///
/// Compute number of outgoing and ingoing edges per node
///
pub fn node_degree(graph: &Gfa) -> Vec<(&str, String)>{
    let mut degree_in: HashMap<&String, u32> = HashMap::new();
    let mut degree_out: HashMap<&String, u32> = HashMap::new();
    let mut degree_total: HashMap<&String, u32> = HashMap::new();
    for x in graph.edges.iter(){
        if degree_in.contains_key(&x.from){
            degree_in.insert(&x.from, degree_in[&x.from]  +1 );
            degree_total.insert(&x.from, degree_in[&x.from]  +1 );

        } else {
            degree_in.insert(&x.from, 1);
            degree_total.insert(&x.from, degree_in[&x.from]  +1 );
        }
        if degree_out.contains_key(&x.to){
            degree_out.insert(&x.to, degree_out[&x.to]  +1 );
            degree_total.insert(&x.from, degree_in[&x.from]  +1 );

        } else {
            degree_out.insert(&x.to, 1);
            degree_total.insert(&x.from, degree_in[&x.from]  +1 );

        }
    }

    let mut result: Vec<(&str, String)> = Vec::new();
    result.push(("Degree in", format!("{:.4}", edges_mean_value(&degree_in))));
    result.push(("Degree out", format!("{:.4}", edges_mean_value(&degree_out))));
    result.push(("Degree total", format!("{:.4}", edges_mean_value(&degree_total))));
    result
}

/// Get mean value from the HashMap(u32, u32) values
///
/// Iterate and sum, then divide by total number of entries
///
pub fn edges_mean_value(hs: &HashMap<&String, u32>) -> f32{
    let mut umax: u32 =0;
    let mut ucout: u32= 0;
    for (_x, y) in hs.iter(){
        ucout += 1;
        umax += y;
    }
    let mean_degree = umax as f32/ucout as f32;
    mean_degree
}

/// Calculate number of inverted edges
/// A egdes going or starting from a negative node
///
/// Maybe need to rethink this///
///
pub fn inverted_edges(graph: &Gfa) -> Vec<(&str, String)>{
    let mut inverted_numb =0;
    for x in graph.edges.iter(){
        if !x.from_dir{
            inverted_numb += 1;
        }
        if !x.to_dir{
            inverted_numb += 1;
        }

    }
    let mut result: Vec<(&str, String)> = Vec::new();
    result.push(("#Inverted nodes", inverted_numb.to_string()));
    result
}



///
/// Get the number of edges and nodes
///
pub fn edges_nodes_number(graph: &Gfa) -> Vec<(&str, String)>{
    let mut result: Vec<(&str, String)> = Vec::new();
    result.push(("#Nodes", graph.nodes.len().to_string()));
    result.push(("#Edges", graph.edges.len().to_string()));
    result

}