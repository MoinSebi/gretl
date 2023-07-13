use gfa_reader::{Gfa, NCGfa, NCGraphWrapper};
use std::collections::{HashMap};
use crate::stats::helper::{mean, median, node_degree};

/// Wrapper for graph statistics
pub fn graph_stats_wrapper(graph: &NCGfa) -> Vec<String>{

    // Result vector
    let mut result = Vec::new();

    // Basic stats
    let path_number = graph_path_number(graph);
    let node_number = graph_node_number(graph);
    let edge_number = graph_edge_number(graph);
    result.push(path_number.to_string());
    result.push(node_number.to_string());
    result.push(edge_number.to_string());

    // Node stats (sizes)
    let (graph_node_average, graph_node_median, graph_node_sum) = graph_node_stats(graph);
    result.push(graph_node_average.to_string());
    result.push(graph_node_median.to_string());
    result.push(graph_node_sum.to_string());

    // Total length of paths
    result.push(graph_path_seq_total(graph).to_string());


    // Node degree
    let (graph_degree_in_average, graph_degree_out_average, graph_degree_total_average) = node_degree(&graph);
    result.push(graph_degree_in_average.to_string());
    result.push(graph_degree_out_average.to_string());
    result.push(graph_degree_total_average.to_string());



    result
}


/// Number of paths
fn graph_path_number(graph: &NCGfa) -> usize{ return graph.paths.len()}

/// Number of nodes
fn graph_node_number(graph: &NCGfa) -> usize{return graph.nodes.len()}

/// Number of edges
fn graph_edge_number(graph: &NCGfa) -> usize {return graph.edges.len()}

/// Calculate total size of all input genomes
pub fn graph_path_seq_total(graph: &NCGfa) ->  usize{
    graph.paths.iter().map(|n| n.nodes.iter().map(|r| graph.nodes[*r as usize]).sum::<usize>()).sum::<usize>()
}




/// Compute mean+median node size and total graph size
///
/// Sum all nodes in the graph and divide it by the number of nodes
pub fn graph_node_stats(graph: &NCGfa) -> (f64, f64, u32){
    let mut vec_size: Vec<u32> = graph.nodes.iter().map(|n| n.1.len as u32).collect();

    vec_size.sort();
    let average = mean(&vec_size);
    let med = median(&mut vec_size);
    let sums: u32 = vec_size.iter().sum();
    (average, med, sums)
}



/// Calculate node degree (in, out, total)
pub fn node_degree2(ncgraph: &NCGraphWrapper,  graph: &NCGfa) -> (f64, f64, f64){
    let (degree_in_values, degree_out_values, degree_total_values) = node_degree(ncgraph, graph);

    let graph_degree_in_average = mean(&degree_in_values);
    let graph_degree_out_average = mean(&degree_out_values);
    let graph_degree_total_average = mean(&degree_total_values);
    (graph_degree_in_average, graph_degree_out_average, graph_degree_total_average)
}

/// Calculate graph density
pub fn graph_desity(graph: &NCGfa) -> f64{
    let n = graph.nodes.len();
    let e = graph.edges.len();
    let density = e as f64 / (n* (n-1)) as f64;
    density
}





#[allow(dead_code)]
/// Calculate number of inverted edges
/// Edges which change direction
pub fn inverted_edges(graph: &NCGfa) -> usize{
    let inverted: usize = graph.edges.iter().filter(|n| n.to_dir != n.from_dir).count();
    inverted
}

