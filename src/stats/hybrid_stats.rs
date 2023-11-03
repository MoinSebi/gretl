use gfa_reader::{GraphWrapper, NCGfa, NCPath};
use crate::stats::helper::{mean, stadard_deviation};
use crate::stats::path_stats::{Arithmetic, mean_path_hm, path_jumps, path_jumps_bigger, path_node_inverted, path_node_len, path_seq_inverted, path_seq_len, path_stats_wrapper, path_unique};

/// Wrapper for path statistics
pub fn path_stats_wrapper2(graph: &NCGfa<()>, gw: &GraphWrapper<NCPath>)  -> Vec<(String, f64)>{



    let path_stats = path_stats_wrapper(graph, gw);

    let mut tmp_res = Vec::new();
    let mut tmp_names = Vec::new();
    for x in path_stats.iter(){
        for y in x.1.iter(){
            tmp_names.push(y.0.clone());
        }
        break
    }

    for x in path_stats.iter(){
        let mut res2 = Vec::new();
        for y in x.1.iter(){
            res2.push(y.1);
        }
        tmp_res.push(res2)
    }
    let mut result = Vec::new();
    for (data, name) in tmp_res.iter().zip(tmp_names.iter()){
        result.push(("Path".to_string() + &name + " (average)", mean(&data.iter().map(|&x| x as u32).collect::<Vec<u32>>())));
        result.push(("Path".to_string() + &name + " (std)", stadard_deviation(&data.iter().map(|&x| x as u32).collect::<Vec<u32>>())));
    }

    return result




}