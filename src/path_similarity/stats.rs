use std::collections::HashMap;
use gfa_reader::{Gfa, GraphWrapper};
use crate::stats::helper::{core1};


/// Compute the amount of sequence in each similarity level
pub fn accession2level(graph: &Gfa, graph_wrapper: &GraphWrapper) -> Vec<(String, Vec<(u32, u32)>)>{
    let cores = core1(graph);
    let metric_maxval = cores.values().max().unwrap();


    let mut result = Vec::new();



    for string2path in graph_wrapper.genomes.iter(){

        let mut result_temp = hashmap_zero(*metric_maxval as usize);
        for path in string2path.1.iter() {
            for node in path.nodes.iter() {
                let metric_value = cores[&node] as usize;
                result_temp.get_mut(&metric_value).unwrap().0 += 1;
                result_temp.get_mut(&metric_value).unwrap().1 += graph.nodes.get(node).unwrap().len as u32;
            }
        }
        // Convert hashmap to vec + add to result
        let result_vec = hashmap2vec(&result_temp);
        result.push((string2path.0.clone(), result_vec));
    }
    return result
}

/// Create hashmap from 0 to n with (0,0) as value
pub fn hashmap_zero(n: usize) -> HashMap<usize, (u32, u32)>{
    let mut hashmap: HashMap<usize, (u32, u32)> = HashMap::with_capacity(n);

    for i in 0..=n {
        hashmap.insert(i, (0,0));
    }
    return hashmap
}

/// HashMap to Vec with (u32, u32) as value and sequential keys (0,1,2,3...)
pub fn hashmap2vec(hashmap: &HashMap<usize, (u32, u32)>) -> Vec<(u32, u32)>{
    let mut result = Vec::with_capacity(hashmap.len());
    for i in 0..hashmap.len() {
        result.push(hashmap[&i]);
    }
    return result
}



