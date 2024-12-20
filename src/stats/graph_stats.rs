use crate::helpers::helper::{average_median_std, mean, median};
use crate::helpers::helper::{calc_depth, calc_node_degree, calc_node_len, calc_similarity};
use crate::stats::hybrid_stats::path_stats_wrapper2;
use gfa_reader::{Gfa, Pansn};
use log::{info, warn};
use std::cmp::max;

/// Wrapper for graph statistics
pub fn graph_stats_wrapper(
    graph: &Gfa<u32, (), ()>,
    wrapper: &Pansn<u32, (), ()>,
    bins: Vec<u32>,
    haplo: bool,
    threads: usize,
) -> Vec<(String, String)> {
    let mut result: Vec<(String, String)> = Vec::new();

    let paths;
    if haplo {
        paths = wrapper.get_haplo_path();
    } else {
        paths = wrapper.get_path_genome();
    }
    let number_samples = wrapper.genomes.len();

    // Basic stats
    let path_number = graph_path_number(graph);
    let node_number = graph_node_number(graph);
    let edge_number = graph_edge_number(graph);
    result.push(("Paths".to_string(), path_number.to_string()));
    result.push(("Samples".to_string(), wrapper.genomes.len().to_string()));
    result.push(("Nodes".to_string(), node_number.to_string()));
    result.push(("Edges".to_string(), edge_number.to_string()));
    result.push((
        "N/E ration".to_string(),
        (node_number as f64 / edge_number as f64).to_string(),
    ));

    // Node stats (sizes)
    let (graph_node_average, graph_node_median, graph_node_sum) = graph_node_stats(graph);
    result.push(("Graph size [bp]".to_string(), graph_node_sum.to_string()));
    let input_seq = graph_path_seq_total(graph);
    result.push(("Input genomes size [bp]".to_string(), input_seq.to_string()));
    result.push((
        "Compression".to_string(),
        (input_seq as f64 / graph_node_sum).to_string(),
    ));

    result.push((
        "Node length (average) [bp]".to_string(),
        graph_node_average.to_string(),
    ));
    result.push((
        "Node length (median) [bp]".to_string(),
        graph_node_median.to_string(),
    ));

    let mut node_size = calc_node_len(graph);

    let size5 = size5pro(&mut node_size);
    result.push((
        "Node length top 5% (median) [bp]".to_string(),
        size5.0.to_string(),
    ));
    result.push((
        "Node length top 5% (average [bp]".to_string(),
        size5.1.to_string(),
    ));

    let nodes1 = bin_nodes_count_and_size(&node_size, bins.clone());

    for x in nodes1.iter() {
        result.push((x.0.to_string(), x.1.to_string()));
    }

    let depth = calc_depth(&paths, graph);
    let core = calc_similarity(&paths, graph);

    // Depth
    let (a1, a2, a3) = average_median_std(&core);
    result.push(("Similarity mean".to_string(), a1.to_string()));
    result.push(("Similarity median".to_string(), a2.to_string()));
    result.push(("Similarity std".to_string(), a3.to_string()));

    result.push((
        "Similarity mean (normalized)".to_string(),
        (a1 / number_samples as f64).to_string(),
    ));
    result.push((
        "Similarity median (normalized)".to_string(),
        (a2 / number_samples as f64).to_string(),
    ));
    result.push((
        "Similarity std (normalized)".to_string(),
        (a3 / number_samples as f64).to_string(),
    ));

    let (a1, a2, a3) = average_median_std(&depth);
    result.push(("Depth mean".to_string(), a1.to_string()));
    result.push(("Depth median".to_string(), a2.to_string()));
    result.push(("Depth std".to_string(), a3.to_string()));

    result.push((
        "Depth mean (normalized)".to_string(),
        (a1 / number_samples as f64).to_string(),
    ));
    result.push((
        "Depth median (normalized)".to_string(),
        (a2 / number_samples as f64).to_string(),
    ));
    result.push((
        "Depth std (normalized)".to_string(),
        (a3 / number_samples as f64).to_string(),
    ));
    // Total length of paths

    // Node degree
    let (graph_degree_in_average, graph_degree_out_average, graph_degree_total_average) =
        calc_node_degree(graph);
    result.push((
        "Node degree (in)".to_string(),
        mean(&graph_degree_in_average).to_string(),
    ));
    result.push((
        "Node degree (out)".to_string(),
        mean(&graph_degree_out_average).to_string(),
    ));
    result.push((
        "Node degree (total)".to_string(),
        mean(&graph_degree_total_average).to_string(),
    ));

    // Inverted edges
    result.push((
        "Inverted edges".to_string(),
        inverted_edges(graph).to_string(),
    ));
    result.push((
        "Inverted edges (normalized)".to_string(),
        (inverted_edges(graph) as f64 / edge_number as f64).to_string(),
    ));

    result.push(("Negative edges".to_string(), neg_edges(graph).to_string()));
    result.push((
        "Negative edges (normalized)".to_string(),
        (neg_edges(graph) as f64 / edge_number as f64).to_string(),
    ));

    result.push(("Self edges".to_string(), self_edge(graph).to_string()));
    result.push((
        "Self edges (normalized)".to_string(),
        (self_edge(graph) as f64 / edge_number as f64).to_string(),
    ));

    // Crazy stuff
    result.push((
        "Graph density".to_string(),
        graph_density(graph).to_string(),
    ));

    if graph.paths.is_empty() {
        warn!("No path or walks found in graph file. No able to run path statistics");
    } else {
        info!("Calculating hybrid stats");
        let hybrid_stats = path_stats_wrapper2(graph, wrapper, haplo, threads);
        for x in hybrid_stats.iter() {
            result.push((x.0.to_string(), x.1.to_string()));
        }
    }

    result
}

/// Number of paths
fn graph_path_number(graph: &Gfa<u32, (), ()>) -> usize {
    graph.paths.len()
}

/// Number of nodes
fn graph_node_number(graph: &Gfa<u32, (), ()>) -> usize {
    graph.segments.len()
}

/// Number of edges
fn graph_edge_number(graph: &Gfa<u32, (), ()>) -> usize {
    graph.links.len()
}

/// Calculate total size of all input genomes
pub fn graph_path_seq_total(graph: &Gfa<u32, (), ()>) -> usize {
    let a = graph
        .paths
        .iter()
        .map(|n| {
            n.nodes
                .iter()
                .map(|r| graph.get_sequence_by_id(r).len())
                .sum::<usize>()
        })
        .sum::<usize>();
    a
}

/// Compute mean+median node size and total graph size
///
/// Sum all nodes in the graph and divide it by the number of nodes
pub fn graph_node_stats(graph: &Gfa<u32, (), ()>) -> (f64, f64, f64) {
    let mut vec_size: Vec<u32> = calc_node_len(graph);
    vec_size.sort();
    let (average, median, _std) = average_median_std(&vec_size);
    let sums = vec_size.iter().sum::<u32>() as f64;
    (average, median, sums)
}

/// Calculate graph density
pub fn graph_density(graph: &Gfa<u32, (), ()>) -> f64 {
    let n = graph.segments.len();
    let e = graph.links.len();

    e as f64 / (n * (n - 1)) as f64
}

/// Calculate number of inverted links
/// Links which change direction
pub fn inverted_edges(graph: &Gfa<u32, (), ()>) -> usize {
    let inverted: usize = graph
        .links
        .iter()
        .filter(|n| n.to_dir != n.from_dir)
        .count();
    inverted
}

/// Number of negative (neg-neg) edges
pub fn neg_edges(graph: &Gfa<u32, (), ()>) -> usize {
    let inverted: usize = graph
        .links
        .iter()
        .filter(|n| (n.to_dir == n.from_dir) && !n.to_dir)
        .count();
    inverted
}

/// Number edges of self stuff
pub fn self_edge(graph: &Gfa<u32, (), ()>) -> usize {
    let inverted: usize = graph
        .links
        .iter()
        .filter(|n| (n.from, n.from_dir) == (n.to, n.to_dir))
        .count();
    inverted
}

/// This is a very simple bin function for graphs LOL
pub fn bin_nodes_count_and_size(value: &Vec<u32>, bins: Vec<u32>) -> Vec<(String, u32)> {
    // These are the bin (with max value)
    let bins: Vec<u32> = bins
        .iter()
        .chain(std::iter::once(&u32::MAX))
        .cloned()
        .collect();

    // Amount of nodes in each bin
    let mut result = vec![0; bins.len()];
    for x in value.iter() {
        for (i, y) in bins.iter().enumerate() {
            if x != &0 && x <= y {
                result[i] += 1;
                break;
            }
        }
    }

    // Write it with the name
    let mut real_result = Vec::new();
    for (i, x) in result.iter().zip(bins.iter()).enumerate() {
        if i == 0 {
            real_result.push(("Bin[0-".to_string() + &x.1.to_string() + "]", *x.0));
        } else {
            real_result.push((
                "Bin[".to_string() + &(bins[i - 1] + 1).to_string() + "-" + &x.1.to_string() + "]",
                *x.0,
            ));
        }
    }
    // Add the last bin
    let value = real_result[real_result.len() - 1]
        .0
        .split('-')
        .next()
        .unwrap()
        .to_string()
        + "-inf]";
    if let Some(last_value) = real_result.last_mut() {
        // Modify the last value (e.g., double it)
        last_value.0 = value;
    }

    real_result
}

/// Top 5 percent of something
pub fn size5pro(f: &mut Vec<u32>) -> (f64, f64) {
    let mut a: Vec<_> = f.to_vec();
    a.retain(|&x| x != 0);
    a.sort_by(|a, b| b.cmp(a));
    let top5 = &a[0..max(1, (a.len() as f64 * 0.05) as usize)].to_vec();

    (median(top5), mean(top5))
}
