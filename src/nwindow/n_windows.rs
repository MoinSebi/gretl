use crate::helpers::helper::node_len;

use gfa_reader::{NCGfa};

use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

pub fn node2node_init(graph: &NCGfa<()>) -> HashMap<u32, HashSet<u32>> {
    let mut map: HashMap<u32, HashSet<u32>> = HashMap::new();

    for element in graph.nodes.iter() {
        map.insert(element.id, HashSet::new());
    }

    let ee = &graph.edges.as_ref().unwrap();

    for x in ee.iter() {
        map.get_mut(&x.from).unwrap().insert(x.to);
        map.get_mut(&x.to).unwrap().insert(x.from);
    }
    map
}

pub fn stats2(
    graph: &NCGfa<()>,
    max_steps: u32,
    max_seq: u32,
    max_jumps: u128,
    return_type: &str,
) -> Vec<[u128; 3]> {
    // Init the node->hs
    let nns = node2node_init(graph);

    // Get an sorted iterator
    let mut nn: Vec<_> = nns.keys().collect();
    nn.sort();

    // Get all the nodes
    let ss = node_len(graph);

    // Result collector
    let mut result: Vec<[u128; 3]> = Vec::with_capacity(nn.len());

    // Iterate over all nodes in order
    for node in nn.iter() {
        let mut seen = HashSet::new();
        let mut res = Vec::new();
        let mut next_next = HashSet::new();
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
        if return_type == "all" {
            let o = [get_nodes(**node, &res) as u128, get_sequence(**node, &res, &ss) as u128, get_jumps(**node, &res)];
            result.push(o);
        } else if return_type == "nodes" {
            let o = [get_nodes(**node, &res) as u128, 0, 0];
            result.push(o);
        } else if return_type == "sequence" {
            let o = [0, get_sequence(**node, &res, &ss) as u128, 0];
            result.push(o);
        } else if return_type == "jumps" {
            let o = [0, 0, get_jumps(**node, &res)];
            result.push(o);
        }
    }

    result
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
            *total_size += sizes[(*y as usize) - 1];
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
        total_sequence += sizes[(*node as usize) - 1];
    }
    total_sequence
}

pub fn get_nodes(_node1: u32, go_to: &Vec<u32>) -> u32 {
    go_to.len() as u32
}
