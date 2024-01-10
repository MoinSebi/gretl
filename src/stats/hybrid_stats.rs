use gfa_reader::{GraphWrapper, NCGfa, NCPath};
use crate::stats::helper::{mean321, stadard_deviation_2};
use crate::stats::path_stats::{path_stats_wrapper};

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
    for x in 0..path_stats[0].1.len(){
        let mut res2 = Vec::new();
        for y in 0..path_stats.len(){
            res2.push(path_stats[y].1[x].1);
        }
        tmp_res.push(res2)
    }


    let mut result = Vec::new();
    for (data, name) in tmp_res.iter().zip(tmp_names.iter()){

        result.push(("Path ".to_string() + &name + " (average)", mean321(&data)));
        result.push(("Path ".to_string() + &name + " (std)", stadard_deviation_2(&data)));
    }

    return result




}