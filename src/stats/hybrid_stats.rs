use gfa_reader::{GraphWrapper, NCGfa, NCPath};
use crate::helpers::helper::{calculate_similarity, calculate_depth, node_degree, node_len, transpose_matrix};
use crate::stats::helper::{mean, stadard_deviation};
use crate::stats::path_stats::{Arithmetic, mean_path_hm, path_jumps, path_jumps_bigger, path_node_inverted, path_node_len, path_seq_inverted, path_seq_len, path_unique};

/// Wrapper for path statistics
pub fn path_stats_wrapper(graph: &NCGfa<()>, gw: &GraphWrapper<NCPath>)  -> Vec<(String, f64)>{

    // Total results
    let mut res = Vec::new();

    // Calculate similarity
    let core = calculate_similarity(&gw, graph);

    // Calculate node degree
    let degree = node_degree(&graph);

    // Calculate node length
    let node_len = node_len(&graph);
    let total_nodes = graph.nodes.len();
    // Calculate depth
    let depth = calculate_depth(&gw, graph);


    let names: Vec<&str> = vec![
        "Path length [bp]",
        "Path nodes",
        "Path unique nodes",
        "Path inverted_nodes",
        "Path inverted_nodes[bp]",
        "Path length_per_node",
        "Path jumps",
        "Path jumps_bigger_than_1000",
        "Path jumps_normalized",
        "Path mean_depth",
        "Path median_depth",
        "Path mean_similarity",
        "Path median_similarity",
        "Path mean_degree",
        "Path median_degree",
        "Nodes_touch",
    ];
    // Iterate over all paths and calculate statistics
    for path in graph.paths.iter(){
        // Temporary results for each path
        let mut result_temp: Vec<f64> = Vec::new();

        // Amount of sequence and number of nodes in the path + number of unique nodes
        let path_len = path_seq_len(path, &graph.nodes);
        let path_nodes = path_node_len(&path.nodes);


        result_temp.push(path_len as f64);
        result_temp.push(path_nodes as f64);
        result_temp.push(path_unique(path) as f64);

        // Number of inverted nodes + their sequence
        result_temp.push(path_node_inverted(path) as f64);
        result_temp.push(path_seq_inverted(path, &graph.nodes) as f64);

        result_temp.push(path_len as f64/path_nodes as f64);
        // Number of jumps - normalized + bigger than x
        let (jumps_total, jumps_normalized) = path_jumps(path);
        result_temp.push(jumps_total as f64);
        result_temp.push(jumps_normalized);

        //result_temp.push(jumps_normalized.to_string());


        let jumps_bigger_than_x = path_jumps_bigger(path, None);
        result_temp.push(jumps_bigger_than_x as f64);

        let mean_depth = mean_path_hm(path, &depth, Arithmetic::MEAN);
        let median_depth = mean_path_hm(path, &depth, Arithmetic::MEDIAN);
        let mean_similarity = mean_path_hm(path, &core, Arithmetic::MEAN);
        let median_similarity = mean_path_hm(path, &core, Arithmetic::MEDIAN);

        // Add to temporary result
        result_temp.push(mean_depth);
        result_temp.push(median_depth);
        result_temp.push(mean_similarity);
        result_temp.push(median_similarity);
        //
        //
        result_temp.push(mean_path_hm(path, &degree.2, Arithmetic::MEAN) as f64);
        result_temp.push(mean_path_hm(path, &degree.2, Arithmetic::MEDIAN) as f64);



        result_temp.push(((path_unique(path) as f64)/(total_nodes as f64)) as f64);

        res.push(result_temp);




    }

    // Transpose the matrix to be able to calculate mean and std
    let a = transpose_matrix(&res);

    let mut result = Vec::new();
    for (name, data) in names.iter().zip(a.iter()) {
        result.push((name.to_string() + " (average)", mean(&data.iter().map(|&x| x as u32).collect::<Vec<u32>>())));
        result.push((name.to_string() + " (std)", stadard_deviation(&data.iter().map(|&x| x as u32).collect::<Vec<u32>>())));
    }

    /
    return result




}