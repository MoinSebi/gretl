use gfa_reader::{Gfa, GraphWrapper, NCEdge, NCGfa, NCPath};
use std::collections::{HashMap};
use crate::helpers::helper::{node_degree, node_len};
use crate::stats::hybrid_stats::path_stats_wrapper;
use crate::stats::helper::{mean, meanf, median};

/// Wrapper for graph statistics
pub fn graph_stats_wrapper(graph: &NCGfa<()>, wrapper: &GraphWrapper<NCPath>, bins: Vec<usize>) -> Vec<(String, String)>{
    let mut wrapper: GraphWrapper<NCPath> = GraphWrapper::new();
    wrapper.from_gfa(&graph.paths, " ");
    // Result vector
    let mut result: Vec<(String, String)>= Vec::new();

    // Basic stats
    let path_number = graph_path_number(graph);
    let node_number = graph_node_number(graph);
    let edge_number = graph_edge_number(graph);
    result.push(("Path_number".to_string(), path_number.to_string()));
    result.push(("Node_number".to_string(), node_number.to_string()));
    result.push(("Edge_number".to_string(), edge_number.to_string()));

    // Node stats (sizes)
    let (graph_node_average, graph_node_median, graph_node_sum) = graph_node_stats(graph);
    result.push(("Node_length_average[bp]".to_string(), graph_node_average.to_string()));
    result.push( ("Node_length_median[bp]".to_string(), graph_node_median.to_string()));
    result.push(("Node_length_sum[bp]".to_string(), graph_node_sum.to_string()));

    // Total length of paths
    result.push(("Input_genomes[bp]".to_string(), graph_path_seq_total(graph).to_string()));


    // Node degree
    let (graph_degree_in_average, graph_degree_out_average, graph_degree_total_average) = node_degree(&graph);
    result.push(("Graph_degree_in".to_string(), mean(&graph_degree_in_average).to_string()));
    result.push(("Graph_degree_out".to_string(), mean(&graph_degree_out_average).to_string()));
    result.push(("Graph_degree_total".to_string(), mean(&graph_degree_total_average).to_string()));

    // Inverted edges
    result.push(("Inverted_edges".to_string(), inverted_edges(graph).to_string()));
    result.push(("Negative_edges".to_string(), neg_edges(graph).to_string()));
    result.push(("Self_edges".to_string(), self_edge(graph).to_string()));

    // Crazy stuff
    result.push(("Graph_density".to_string(), graph_density(graph).to_string()));

    let node_size = node_len(graph);
    let nodes1 = bin_nodes_count_and_size(node_size, vec![1,50,100,1000,5000]);

    for x in nodes1.iter(){
        result.push((x.0.to_string(), x.1.to_string()));
    }

    let size5 = size5pro(graph);
    result.push(("Max_node_size".to_string(), size5.0.to_string()));
    result.push(("Average_node_size".to_string(), size5.1.to_string()));


    let hybrid_stats = path_stats_wrapper(graph, &wrapper);
    for x in hybrid_stats.iter(){
        result.push((x.0.to_string(), x.1.to_string()));
    }

    result
}


/// Number of paths
fn graph_path_number(graph: &NCGfa<()>) -> usize{ return graph.paths.len()}

/// Number of nodes
fn graph_node_number(graph: &NCGfa<()>) -> usize{return graph.nodes.len()}

/// Number of edges
fn graph_edge_number(graph: &NCGfa<()>) -> usize {return graph.edges.as_ref().unwrap().len()}

/// Calculate total size of all input genomes
pub fn graph_path_seq_total(graph: &NCGfa<()>) ->  usize{
    let a = graph.paths.iter().map(|n| n.nodes.iter().map(|r| graph.nodes[*r as usize - 1].seq.len()).sum::<usize>()).sum::<usize>();
    a
}




/// Compute mean+median node size and total graph size
///
/// Sum all nodes in the graph and divide it by the number of nodes
pub fn graph_node_stats(graph: &NCGfa<()>) -> (f64, f64, u32){

    let mut vec_size: Vec<u32> = graph.nodes.iter().map(|n| n.seq.len() as u32).collect();

    vec_size.sort();
    let average = mean(&vec_size);
    let med = median(&mut vec_size);
    let sums: u32 = vec_size.iter().sum();
    (average, med, sums)
}



/// Calculate node degree (in, out, total)
pub fn node_degree2(ncgraph: &GraphWrapper<NCPath>,  graph: &NCGfa<()>) -> (f64, f64, f64){
    let (degree_in_values, degree_out_values, degree_total_values) = node_degree(graph);

    let graph_degree_in_average = mean(&degree_in_values);
    let graph_degree_out_average = mean(&degree_out_values);
    let graph_degree_total_average = mean(&degree_total_values);
    (graph_degree_in_average, graph_degree_out_average, graph_degree_total_average)
}

/// Calculate graph density
pub fn graph_density(graph: &NCGfa<()>) -> f64{
    let n = graph.nodes.len();
    let e = graph.edges.as_ref().unwrap().len();
    let density = e as f64 / (n* (n-1)) as f64;
    density
}





#[allow(dead_code)]
/// Calculate number of inverted edges
/// Edges which change direction
pub fn inverted_edges(graph: &NCGfa<()>) -> usize{
    let inverted: usize = graph.edges.as_ref().unwrap().iter().filter(|n| n.to_dir != n.from_dir).count();
    inverted
}


/// Number of negative (neg-neg) edges
pub fn neg_edges(graph: &NCGfa<()>) -> usize{
    let inverted: usize = graph.edges.as_ref().unwrap().iter().filter(|n| (n.to_dir == n.from_dir) && (n.to_dir == false)).count();
    inverted
}


/// Number edges of self stuff
pub fn self_edge(graph: &NCGfa<()>) -> usize{
    let inverted: usize = graph.edges.as_ref().unwrap().iter().filter(|n| (n.from, n.from_dir) == (n.to, n.to_dir)).count();
    inverted
}


/// This is a very simple bin function for graphs LOL
pub fn bin_nodes_count_and_size(value: Vec<u32>, bins: Vec<u32>) -> Vec<(String, u32)>{

    let bins:Vec<u32> = bins.iter().chain(std::iter::once(&u32::MAX)).cloned().collect();
    let mut result = vec![0; bins.len()];
    for x in value.iter() {
        for (i, y) in bins.iter().enumerate() {
            if x < y {
                result[i] += 1;
            }
        }
    }
    let mut real_result = Vec::new();
    for (i, x) in result.iter().zip(bins.iter()).enumerate(){
        if i == 0{
            real_result.push(("Bin[0-".to_string() + &x.1.to_string() + &"]".to_string(), x.1.clone()));
        } else {
            real_result.push(("Bin[".to_string() + &bins[i-1].to_string() + &"-".to_string() + &x.1.to_string() + &"]".to_string(), x.1.clone()));

        }
    }
    let value = real_result[real_result.len()-1].0.split("-").next().unwrap().to_string() + &"-inf]".to_string();
    if let Some(last_value) = real_result.last_mut() {
        // Modify the last value (e.g., double it)
        last_value.0 = value;
    }

    real_result
}



/// Number edges of self stuff
pub fn node_size_each_bin(graph: &NCGfa<()>) -> [[usize; 4]; 2]{
    let mut size_1 = 0;
    let mut size_1_50 = 0;
    let mut size_50_1000 = 0;
    let mut size_1000 = 0;


    let mut ssize_1 = 0;
    let mut ssize_1_50 = 0;
    let mut ssize_50_1000 = 0;
    let mut ssize_1000 = 0;

    for x in graph.nodes.iter(){
        let length = x.seq.len();
        if length == 1{
            size_1 += 1;
            ssize_1 += length;

        } else if length < 50 {
            size_1_50 += 1;
            ssize_1_50 += length;

        } else if length < 1000{
            size_50_1000 += 1;
            ssize_50_1000 += length;

        } else {
            size_1000 += 1;
            ssize_1000 += length;

        }
    }

    [[size_1, size_1_50, size_50_1000, size_1000], [ssize_1, ssize_1_50, ssize_50_1000, ssize_1000]]

}


pub fn size5pro(graph: &NCGfa<()>) -> (usize, f64) {
    let mut a: Vec<usize> = graph.nodes.iter().map(|n| n.seq.len()).collect();
    a.sort();
    let top5: &[usize] = &a[0..(a.len() as f64 * 0.05) as usize];

    let maxx = a.iter().max().unwrap();

    (*maxx, mean(&top5.iter().map(|n| *n as u32).collect::<Vec<u32>>()))
}


