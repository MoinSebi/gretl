use gfa_reader::{Gfa, GraphWrapper, NCEdge, NCGfa, NCPath};
use std::collections::{HashMap};
use crate::helpers::helper::{calculate_similarity, calculate_depth, node_degree, node_len};
use crate::stats::hybrid_stats::path_stats_wrapper;
use crate::stats::helper::{average_median_std, mean, meanf, median};

/// Wrapper for graph statistics
pub fn graph_stats_wrapper(graph: &NCGfa<()>, wrapper: &GraphWrapper<NCPath>, bins: Vec<usize>) -> Vec<(String, String)>{
    let mut wrapper: GraphWrapper<NCPath> = GraphWrapper::new();
    wrapper.from_gfa(&graph.paths, " ");
    // Result vector
    let mut result: Vec<(String, String)>= Vec::new();

    let mut depth = calculate_depth(&wrapper, graph);
    let mut core = calculate_similarity(&wrapper, graph);

    // Basic stats
    let path_number = graph_path_number(graph);
    let node_number = graph_node_number(graph);
    let edge_number = graph_edge_number(graph);
    result.push(("#Paths".to_string(), path_number.to_string()));
    result.push(("#Samples".to_string(), wrapper.genomes.len().to_string()));
    result.push(("#Nodes".to_string(), node_number.to_string()));
    result.push(("#Edges".to_string(), edge_number.to_string()));
    result.push(("N/E ration".to_string(), (node_number as f64/edge_number as f64).to_string()));

    // Node stats (sizes)
    let (graph_node_average, graph_node_median, graph_node_sum) = graph_node_stats(graph);
    result.push(("Node length (average) [bp]".to_string(), graph_node_average.to_string()));
    result.push( ("Node length (median) [bp]".to_string(), graph_node_median.to_string()));
    result.push(("Node length  (sum) [bp]".to_string(), graph_node_sum.to_string()));

    let mut node_size = node_len(graph);

    let size5 = size5pro(& mut node_size);
    result.push(("Node length (max) [bp]".to_string(), size5.0.to_string()));
    result.push(("Node length (average, top 5%) [bp]".to_string(), size5.1.to_string()));

    let nodes1 = bin_nodes_count_and_size(&node_size, vec![1,50,100,1000,5000]);

    for x in nodes1.iter(){
        result.push((x.0.to_string(), x.1.to_string()));
    }



    // Depth
    let (a1,a2, a3) = average_median_std(&mut core);
    result.push( ("Similarity (mean)".to_string(), a1.to_string()));
    result.push(("Similarity (median)".to_string(), a2.to_string()));
    result.push(("Similarity (std)".to_string(), a2.to_string()));
    let (a1,a2, a3) = average_median_std(&mut depth);
    result.push( ("Depth (mean)".to_string(), a1.to_string()));
    result.push(("Depth (median)".to_string(), a2.to_string()));
    result.push(("Depth (std)".to_string(), a2.to_string()));
    // Total length of paths
    let input_seq = graph_path_seq_total(graph);
    result.push(("Input genomes size [bp]".to_string(), input_seq.to_string()));
    result.push(("Compression".to_string(), (input_seq as f64/graph_node_sum as f64).to_string()));


    // Node degree
    let (graph_degree_in_average, graph_degree_out_average, graph_degree_total_average) = node_degree(&graph);
    result.push(("Node degree (in)".to_string(), mean(&graph_degree_in_average).to_string()));
    result.push(("Node degree (out)".to_string(), mean(&graph_degree_out_average).to_string()));
    result.push(("Node degree (total)".to_string(), mean(&graph_degree_total_average).to_string()));

    // Inverted edges
    result.push(("#Inverted edges".to_string(), inverted_edges(graph).to_string()));
    result.push(("#Negative edges".to_string(), neg_edges(graph).to_string()));
    result.push(("#Self edges".to_string(), self_edge(graph).to_string()));

    // Crazy stuff
    result.push(("Graph density".to_string(), graph_density(graph).to_string()));



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
pub fn bin_nodes_count_and_size(value: &Vec<u32>, bins: Vec<u32>) -> Vec<(String, u32)>{

    // These are the bin (with max value)
    let bins:Vec<u32> = bins.iter().chain(std::iter::once(&u32::MAX)).cloned().collect();

    // Amount of nodes in each bin
    let mut result = vec![0; bins.len()];
    for x in value.iter() {
        for (i, y) in bins.iter().enumerate() {
            if x < y {
                result[i] += 1;
            }
        }
    }

    // Write it with the name
    let mut real_result = Vec::new();
    for (i, x) in result.iter().zip(bins.iter()).enumerate(){
        if i == 0{
            real_result.push(("Bin[0-".to_string() + &x.1.to_string() + &"]".to_string(), x.1.clone()));
        } else {
            real_result.push(("Bin[".to_string() + &bins[i-1].to_string() + &"-".to_string() + &x.1.to_string() + &"]".to_string(), x.1.clone()));

        }
    }
    // Add the last bin
    let value = real_result[real_result.len()-1].0.split("-").next().unwrap().to_string() + &"-inf]".to_string();
    if let Some(last_value) = real_result.last_mut() {
        // Modify the last value (e.g., double it)
        last_value.0 = value;
    }

    real_result
}

/// Top 5 percent of something
pub fn size5pro(f: &mut Vec<u32>) -> (usize, f64) {
    let mut a = f.iter().map(|n| *n as usize).collect::<Vec<usize>>();
    a.sort();
    let top5: &[usize] = &a[0..(a.len() as f64 * 0.05) as usize];

    let maxx = a.iter().max().unwrap();

    (*maxx, mean(&top5.iter().map(|n| *n as u32).collect::<Vec<u32>>()))
}


