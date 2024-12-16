use crate::helpers::helper::calc_node_len;

use gfa_reader::Gfa;
use log::info;
use rayon::prelude::*;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::sync::atomic::AtomicUsize;

/// Shows edges
/// Node to node connection
pub fn node2node_init(graph: &Gfa<u32, (), ()>) -> HashMap<u32, HashSet<u32>> {
    let mut map: HashMap<u32, HashSet<u32>> = HashMap::new();

    for element in graph.segments.iter() {
        map.insert(element.id, HashSet::new());
    }

    let ee = &graph.links;

    for x in ee.iter() {
        map.get_mut(&x.from).unwrap().insert(x.to);
        map.get_mut(&x.to).unwrap().insert(x.from);
    }
    map
}

pub fn nwindow_wrapper(
    graph: &Gfa<u32, (), ()>,
    max_steps: u32,
    max_seq: u32,
    max_jumps: u128,
    _return_type: &str,
    threads: usize,
) -> Vec<[u128; 4]> {
    // Init the node->hs

    info!("Create HashMap with node to node connections");
    let nns = node2node_init(graph);

    // Get a sorted iterator
    let mut nn: Vec<_> = nns.keys().collect();
    nn.sort();

    // Get all the nodes
    let ss = calc_node_len(graph);

    let count = AtomicUsize::new(0);
    // Result collector
    let chunk_size = (nn.len() + threads - 1) / threads;

    info!("Start the parallel computation");
    let result_network: Vec<[u128; 4]> = nn
        .par_chunks(chunk_size)
        .flat_map(|chunk| {
            let mut result = Vec::new();
            for node in chunk {
                let mut seen = HashSet::new();
                let mut res = Vec::new();
                let mut next_next: HashSet<_>;
                let mut nexts = nns[node].clone();
                res.push(**node);

                let mut sum_sequence = 0;
                let mut steps = 0;
                let mut sum_jumps = 0;
                while steps < max_steps && sum_sequence < max_seq && sum_jumps < max_jumps {
                    next_next = iterator(
                        &mut seen,
                        &nns,
                        &mut nexts,
                        &mut res,
                        &ss,
                        &mut sum_sequence,
                        node,
                        &mut sum_jumps,
                    );
                    nexts = next_next;
                    steps += 1;
                }
                let network_stats = [
                    get_nodes(**node, &res) as u128,
                    get_sequence(**node, &res, &ss) as u128,
                    get_jumps(**node, &res),
                    **node as u128,
                ];
                result.push(network_stats);
                count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                std::io::stderr().flush().unwrap();
                eprint!(
                    "\r{}          Progress {:.2}%",
                    chrono::Local::now().format("%d/%m/%Y %H:%M:%S %p"),
                    (count.load(std::sync::atomic::Ordering::Relaxed) as f64 / nn.len() as f64)
                        * 100.0
                );
            }
            result
        })
        .collect();
    eprintln!();

    result_network
}

pub fn iterator(
    seen: &mut HashSet<u32>,
    nodes2nodes: &HashMap<u32, HashSet<u32>>,
    nexts: &mut HashSet<u32>,
    result: &mut Vec<u32>,
    sizes: &Vec<u32>,
    total_size: &mut u32,
    last_node: &u32,
    total_jumps: &mut u128,
) -> HashSet<u32> {
    let mut next_nodes = HashSet::new();

    for y in nexts.iter() {
        if !seen.contains(y) {
            seen.insert(*y);
            next_nodes.extend(&nodes2nodes[y]);
            result.push(*y);
            *total_size += sizes[*y as usize];
            *total_jumps += (max(last_node, y) - min(last_node, y)) as u128;
        }
    }
    next_nodes = next_nodes.difference(seen).cloned().collect();
    next_nodes
}

pub fn get_jumps(node1: u32, go_to: &Vec<u32>) -> u128 {
    let mut total_jumps = 0;
    for node in go_to.iter() {
        total_jumps += (max(*node, node1) - min(*node, node1)) as u128;
    }
    total_jumps
}

pub fn get_sequence(_node1: u32, go_to: &Vec<u32>, sizes: &Vec<u32>) -> u32 {
    let mut total_sequence = 0;
    for node in go_to.iter() {
        total_sequence += sizes[*node as usize];
    }
    total_sequence
}

pub fn get_nodes(_node1: u32, go_to: &Vec<u32>) -> u32 {
    go_to.len() as u32
}
