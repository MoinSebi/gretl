use gfa_reader::{GraphWrapper, NCGfa, NCPath};
use crate::helpers::helper::{calculate_core, calculate_depth, node_degree};
use crate::stats::helper::{mean, std};
use crate::stats::path_stats::{Arithmetic, mean_path_hm, path_jumps, path_jumps_bigger, path_node_inverted, path_node_len, path_seq_inverted, path_seq_len, path_unique};

/// Wrapper for path statistics
pub fn path_stats_wrapper(graph: &NCGfa<()>, gw: &GraphWrapper<NCPath>)  -> (Vec<f64>, Vec<f64>){

    // Total results
    let mut res = Vec::new();

    // Calculate similarity
    let core = calculate_core(&gw, graph);

    // Calculate node degree
    let test = node_degree(&graph);

    // Calculate depth
    let depth = calculate_depth(&gw, graph);

    // Iterate over all paths and calculate statistics
    for path in graph.paths.iter(){
        // Temporary results for each path
        let mut result_temp = Vec::new();

        // Amount of sequence and number of nodes in the path + number of unique nodes
        result_temp.push(path_seq_len(path, &graph.nodes));
        result_temp.push(path_node_len(&path.nodes));
        result_temp.push(path_unique(path));

        // Number of inverted nodes + their sequence
        result_temp.push(path_node_inverted(path));
        result_temp.push(path_seq_inverted(path, &graph.nodes));

        // Number of jumps - normalized + bigger than x
        let (jumps_total, jumps_normalized) = path_jumps(path);
        result_temp.push(jumps_total);

        //result_temp.push(jumps_normalized.to_string());


        let jumps_bigger_than_x = path_jumps_bigger(path, None);
        result_temp.push(jumps_bigger_than_x as usize);

        // let mean_depth = mean_path_hm(path, &depth, Arithmetic::MEAN);
        // let median_depth = mean_path_hm(path, &depth, Arithmetic::MEDIAN);
        // let mean_similarity = mean_path_hm(path, &core, Arithmetic::MEAN);
        // let median_similarity = mean_path_hm(path, &core, Arithmetic::MEDIAN);
        //
        // // Add to temporary result
        // result_temp.push(mean_depth.to_string());
        // result_temp.push(median_depth.to_string());
        // result_temp.push(mean_similarity.to_string());
        // result_temp.push(median_similarity.to_string());
        //
        //
        // result_temp.push(mean_path_hm(path, &test.2, Arithmetic::MEAN).to_string());
        // result_temp.push("test".to_string());
        res.push(result_temp)

    }

    let mut p = Vec::new();
    let mut p2 = Vec::new();
    for i in 0..res.len(){
        let mut temp = Vec::new();
        for y in 0..res[0].len(){
            temp.push(res[i][y] as u32)
        }
        p2.push(std(&temp));
        p.push(mean(&temp));
    }
    (p, p2)

}