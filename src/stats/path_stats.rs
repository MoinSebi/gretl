use crate::helpers::helper::{calculate_depth, calculate_similarity, node_degree, node_len};
use crate::stats::helper::{mean_usize, median2, std_usize};
use gfa_reader::{NCGfa, NCNode, NCPath, Pansn};
use std::collections::HashSet;

/// Wrapper for path statistics
///
/// Output is a vector of [pathname, vec<(stat_name, value)>]
pub fn path_stats_wrapper(
    graph: &NCGfa<()>,
    wrapper: &Pansn<NCPath>,
    haplo: bool,
) -> Vec<(String, Vec<(String, f64)>)> {
    // Total results
    let mut res = Vec::new();
    let paths;
    if haplo {
        paths = wrapper.get_haplo_path();
    } else {
        paths = wrapper.get_path_genome();
    }
    let _number_samples = wrapper.genomes.len();

    // Calculate similarity
    let core = calculate_similarity(&paths, graph);

    // Calculate node degree
    let node_degree = node_degree(graph);

    // Calculate depth
    let depth = calculate_depth(&paths, graph);
    let node_size = node_len(graph);

    // Iterate over all paths and calculate statistics
    for path in paths.iter() {
        // We normalize everything by node number and node length
        let mut result_temp: Vec<(String, f64)> = Vec::new();
        let path_seq = path_seq_len(&path.1, &graph.nodes) as f64;
        let path_nodes = path_node_len(&path.1) as f64;
        let dir_nodes = dir_node(&path.1) as f64;
        let edges = edges_num(&path.1);
        let edges_total_numb = path_nodes - 1.0;

        // Amount of sequence and number of nodes in the path + number of unique nodes
        result_temp.push(("Sequence [bp]".to_string(), path_seq));
        result_temp.push(("Nodes".to_string(), path_nodes));
        result_temp.push(("Unique edges".to_string(), dir_nodes));
        // Dumb info, but well
        result_temp.push(("Directed nodes".to_string(), dir_nodes));

        result_temp.push(("Edges".to_string(), edges.0));
        result_temp.push(("Unique Edges".to_string(), edges.1));

        let path_unique_val = path_unique2(&path.1, &graph.nodes);
        result_temp.push(("Unique nodes".to_string(), path_unique_val.0 as f64));
        result_temp.push(("Unique nodes [bp]".to_string(), path_unique_val.1 as f64));

        result_temp.push((
            "Unique nodes (normalized)".to_string(),
            path_unique_val.0 as f64 / path_nodes,
        ));
        result_temp.push((
            "Unique nodes [bp] (normalized)".to_string(),
            path_unique_val.1 as f64 / path_seq,
        ));

        result_temp.push((
            "Unique edges (normalized)".to_string(),
            dir_nodes / edges_total_numb,
        ));

        let inverted = path_seq_inverted(&path.1, &graph.nodes);

        result_temp.push(("Inverted nodes".to_string(), inverted.0 as f64));
        result_temp.push(("Inverted nodes [bp]".to_string(), inverted.1 as f64));

        result_temp.push((
            "Inverted nodes (normalized)".to_string(),
            inverted.0 as f64 / path_nodes,
        ));
        result_temp.push((
            "Inverted nodes [bp] (normalized)".to_string(),
            inverted.1 as f64 / path_seq,
        ));

        // Number of jumps - normalized + bigger than x
        let jumps_total = path_jumps(&path.1);
        result_temp.push(("Jumps total".to_string(), jumps_total as f64));
        result_temp.push((
            "Jumps_total (normalized)".to_string(),
            jumps_total as f64 / edges_total_numb,
        ));

        let jumps_bigger_than_x = path_jumps_bigger(&path.1, None);
        result_temp.push((
            "Jumps bigger than X".to_string(),
            jumps_bigger_than_x as f64,
        ));
        result_temp.push((
            "Jumps bigger than X (normalized)".to_string(),
            jumps_bigger_than_x as f64 / edges_total_numb,
        ));

        let (node_sizes_avg, node_size_median, node_size_std) = node_size_cal(&path.1, &node_size);
        result_temp.push(("Node size average [bp]".to_string(), node_sizes_avg));
        result_temp.push(("Node size median [bp]".to_string(), node_size_median));
        result_temp.push(("Node size std [bp]".to_string(), node_size_std));

        let (depth_avg, depth_median, depth_std) = node_size_cal(&path.1, &depth);
        result_temp.push((
            "Depth average".to_string(),
            depth_avg / wrapper.genomes.len() as f64,
        ));
        result_temp.push((
            "Depth median".to_string(),
            depth_median / wrapper.genomes.len() as f64,
        ));
        result_temp.push((
            "Depth std".to_string(),
            depth_std / wrapper.genomes.len() as f64,
        ));

        result_temp.push((
            "Depth average (normalized)".to_string(),
            depth_avg / wrapper.genomes.len() as f64,
        ));
        result_temp.push((
            "Depth median (normalized)".to_string(),
            depth_median / wrapper.genomes.len() as f64,
        ));
        result_temp.push((
            "Depth std (normalized)".to_string(),
            depth_std / wrapper.genomes.len() as f64,
        ));

        let (sim_avg, sim_median, sim_std) = node_size_cal(&path.1, &core);
        result_temp.push(("Similarity average".to_string(), sim_avg));
        result_temp.push(("Similarity median".to_string(), sim_median));
        result_temp.push(("Similarity std".to_string(), sim_std));

        result_temp.push((
            "Similarity average (normalized)".to_string(),
            sim_avg / wrapper.genomes.len() as f64,
        ));
        result_temp.push((
            "Similarity median (normalized)".to_string(),
            sim_median / wrapper.genomes.len() as f64,
        ));
        result_temp.push((
            "Similarity std (normalized)".to_string(),
            sim_std / wrapper.genomes.len() as f64,
        ));

        let (degree_avg, degree_median, degree_std) = node_size_cal(&path.1, &node_degree.2);
        result_temp.push(("Degree average".to_string(), degree_avg));
        result_temp.push(("Degree median".to_string(), degree_median));
        result_temp.push(("Degree std".to_string(), degree_std));

        res.push((path.0.to_string(), result_temp))
    }

    res
}

pub fn remove_unsorted(input: &mut Vec<(String, Vec<(String, String)>)>, graph: &NCGfa<()>) {
    if !graph.check_numeric() {
        for x in 0..input.len() {
            input[x].1.retain(|m| !m.0.starts_with("Jump"))
        }
    }
}

pub fn get_all_stats(input: &Vec<usize>) -> (f64, f64, f64) {
    let mean = mean_usize(input);
    let median = median2(input) as f64;
    let std = std_usize(input);

    (mean, median, std)
}

/// Calculate the noed size
pub fn node_size_cal(path: &Vec<&NCPath>, node_sizes: &Vec<u32>) -> (f64, f64, f64) {
    let mut result = Vec::new();
    for p in path.iter() {
        for x in p.nodes.iter() {
            result.push(node_sizes[*x as usize - 1] as usize)
        }
    }
    let (mean, median, std) = get_all_stats(&result);

    (mean, median, std)
}

/// Count the number of nodes for each path
pub fn path_node_len(path: &Vec<&NCPath>) -> usize {
    let pp = path.iter().map(|n| n.nodes.len()).sum();
    pp
}

/// Calculate the length of path
pub fn path_seq_len(path: &Vec<&NCPath>, nodes: &Vec<NCNode<()>>) -> usize {
    let size: usize = path
        .iter()
        .map(|_n| nodes.iter().map(|nn| nn.seq.len()).sum::<usize>())
        .sum::<usize>();
    size
}

pub fn dir_node(path: &Vec<&NCPath>) -> usize {
    let edges2: HashSet<(&u32, &bool)> = path
        .iter()
        .flat_map(|l| l.nodes.iter().zip(l.dir.iter()))
        .collect();
    edges2.len()
}

pub fn edges_num(path: &Vec<&NCPath>) -> (f64, f64) {
    let edges2: Vec<_> = path
        .iter()
        .flat_map(|l| {
            l.nodes
                .iter()
                .zip(l.dir.iter())
                .skip(1)
                .zip(l.nodes.iter().zip(l.dir.iter()))
        })
        .collect();
    let p: HashSet<_> = edges2.iter().collect();
    (edges2.len() as f64, p.len() as f64)
}

#[allow(dead_code)]
/// Count the number of inverted nodes for each path
pub fn path_node_inverted(path: &NCPath) -> usize {
    path.dir.iter().filter(|&n| !(*n)).count()
}

#[allow(dead_code)]
/// Count the number of inverted nodes for each path
pub fn path_seq_inverted(path: &Vec<&NCPath>, nodes: &Vec<NCNode<()>>) -> (usize, usize) {
    let inverted = path
        .iter()
        .map(|n| n.dir.iter().filter(|&n| !(*n)).count())
        .sum();
    let sums: usize = path
        .iter()
        .map(|n| {
            n.dir
                .iter()
                .zip(&n.nodes)
                .filter(|&n| !(*n.0))
                .map(|s| nodes.get(*s.1 as usize - 1).unwrap().seq.len())
                .sum::<usize>()
        })
        .sum();
    (inverted, sums)
}

/// Calculate the total number of jumps
///
/// Return:
/// - total number of jumps
/// - total number of jumps divided by the number of nodes
///
/// TODO
/// - running average (no overflow)
pub fn path_jumps(path: &Vec<&NCPath>) -> usize {
    let mut c: usize = 0;
    for p in path.iter() {
        let mut last = p.nodes[0];
        for node_id in p.nodes.iter().skip(1) {
            c += (*node_id as i64 - last as i64).unsigned_abs() as usize;
            last = *node_id
        }
    }
    c
}

/// Count the number of jumps bigger than X
pub fn path_jumps_bigger(path: &Vec<&NCPath>, val: Option<i32>) -> u32 {
    let distance = val.unwrap_or(20);
    let mut c: u32 = 0;
    for p in path.iter() {
        let mut last = 0;
        for x in p.nodes.iter() {
            let ff: i32 = *x as i32 - last as i32;
            last = *x;
            if ff.abs() > distance {
                c += 1
            }
        }
    }

    c
}

pub fn path_unique2(path: &Vec<&NCPath>, nodes: &Vec<NCNode<()>>) -> (usize, usize) {
    let hp: HashSet<_> = path.iter().flat_map(|n| n.nodes.iter()).collect();

    let unique_seq = hp.iter().map(|x| nodes[**x as usize - 1].seq.len()).sum();
    (hp.len(), unique_seq)
}

#[allow(dead_code)]
/// Calculate the number of repeated nodes
pub fn path_cycle(path: &NCPath) -> usize {
    let mut _c = 0;
    let mut hs: HashSet<&u32> = HashSet::new();
    for x in path.nodes.iter() {
        if hs.contains(x) {
            _c += 1
        }
        hs.insert(x);
    }
    _c
}

pub fn convert_data(
    input: &mut Vec<(String, Vec<(String, f64)>)>,
) -> Vec<(String, Vec<(String, String)>)> {
    let mut res = Vec::new();
    for x in input.iter_mut() {
        let mut temp = Vec::new();
        for y in x.1.iter_mut() {
            temp.push((y.0.to_string(), y.1.to_string()))
        }
        res.push((x.0.to_string(), temp))
    }

    res
}
