use std::collections::{HashMap, HashSet};
use gfa_reader::NGfa;

/// Counting the amount of accessions and depth
pub fn calculate_core(graph: &NGfa) -> HashMap<u32, u32>{
    let mut count: HashMap<u32, u32> = HashMap::new();
    for x in &graph.nodes{
        count.insert(x.0.clone(), 0);
    }

    for x in &graph.paths{

        let v: HashSet<_> = x.nodes.iter().cloned().collect();
        for y in v{
            *count.get_mut(&y).unwrap() += 1;
        }
    }
    count.shrink_to_fit();
    count
}

/// Counting the amount of accessions and depth
pub fn calculate_depth(graph: &NGfa) -> HashMap<u32, u32>{
    let mut count: HashMap<u32, u32> = HashMap::new();
    for x in &graph.nodes{
        count.insert(x.0.clone(), 0);
    }

    for path in &graph.paths{


        for y in path.nodes.iter(){
            *count.get_mut(&y).unwrap() += 1;
        }
    }
    count.shrink_to_fit();
    count
}



pub fn mean(data: Vec<u32>) -> f64{
    let sums = data.iter().sum() as f64/data.iter().len() as f64;
    return sums
}

pub fn median(data: Ve)

pub fn depth(graph: &NGfa, counts: ){
    let count = counting_graph(graph);
    for path in graph.paths.iter(){
        let mut data = Vec::new();
        for p in path.iter(){

        }
    }
}
