use gfa_reader::Gfa;
use std::collections::{HashMap};
use crate::stats::helper::{mean, median};

pub fn graph_stats_wrapper(graph: &Gfa) -> Vec<String>{
    let mut result = Vec::new();
    result.push(graph_path_number(graph).to_string());
    result.push(graph_node_number(graph).to_string());
    result.push(graph_edge_number(graph).to_string());

    let (graph_node_average, graph_node_median, graph_node_sum) = graph_node_stats(graph);
    result.push(graph_node_average.to_string());
    result.push(graph_node_median.to_string());
    result.push(graph_node_sum.to_string());

    result.push(graph_path_seq_total(graph).to_string());


    let (graph_degree_in_average, graph_degree_out_average, graph_degree_total_average) = node_degree(&graph);
    result.push(graph_degree_in_average.to_string());
    result.push(graph_degree_out_average.to_string());
    result.push(graph_degree_total_average.to_string());



    result
}

/// Number of paths
fn graph_path_number(graph: &Gfa) -> usize{ return graph.paths.len()}

/// Number of nodes
fn graph_node_number(graph: &Gfa) -> usize{return graph.nodes.len()}

/// Number of edges
fn graph_edge_number(graph: &Gfa) -> usize {return graph.edges.len()}

/// Calculate total size of all input genomes
pub fn graph_path_seq_total(graph: &Gfa) ->  usize{
    graph.paths.iter().map(|n| n.nodes.iter().map(|r| graph.nodes[r].len).sum::<usize>()).sum::<usize>()
}




/// Compute mean+median node size and total graph size
///
/// Sum all nodes in the graph and divide it by the number of nodes
pub fn graph_node_stats(graph: &Gfa) -> (f64, u32, u32){
    let mut vec_size: Vec<u32> = graph.nodes.iter().map(|n| n.1.len as u32).collect();

    vec_size.sort();
    let average = mean(&vec_size);
    let med = median(&mut vec_size);
    let sums: u32 = vec_size.iter().sum();
    (average, med, sums)
}



/// Calculate node degree (in, out, total)
pub fn node_degree(graph: &Gfa) -> (f64, f64, f64){
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
    let degree_in_values: Vec<u32> = degree_in.into_values().collect();
    let degree_out_values: Vec<u32> = degree_out.into_values().collect();
    let degree_total_values: Vec<u32> = degree_total.into_values().collect();

    let graph_degree_in_average = mean(&degree_in_values);
    let graph_degree_out_average = mean(&degree_out_values);
    let graph_degree_total_average = mean(&degree_total_values);
    (graph_degree_in_average, graph_degree_out_average, graph_degree_total_average)
}


#[allow(dead_code)]
/// Calculate number of inverted edges
/// Edges which change direction
pub fn inverted_edges(graph: &Gfa) -> usize{
    let inverted: usize = graph.edges.iter().filter(|n| n.to_dir != n.from_dir).count();
    inverted
}