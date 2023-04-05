use std::collections::{HashMap, HashSet};
use gfa_reader::{Gfa, NGfa};

/// Counting the amount of accessions and depth
pub fn calculate_core(graph: &Gfa) -> HashMap<u32, u32>{
    let mut count: HashMap<u32, u32> = HashMap::new();
    for x in &graph.nodes{
        count.insert(x.0.parse().unwrap(), 0);
    }

    for x in &graph.paths{

        let v: HashSet<_> = x.nodes.iter().cloned().collect();
        for y in v{
            *count.get_mut(&y.parse().unwrap()).unwrap() += 1;
        }
    }
    count.shrink_to_fit();
    count
}

/// Counting the amount of accessions and depth
pub fn calculate_depth(graph: &Gfa) -> HashMap<u32, u32>{
    let mut depth: HashMap<u32, u32> = HashMap::new();
    for x in &graph.nodes{
        depth.insert(x.0.parse().unwrap(), 0);
    }

    for path in &graph.paths{


        for y in path.nodes.iter(){
            *depth.get_mut(&y.parse().unwrap()).unwrap() += 1;
        }
    }
    depth.shrink_to_fit();
    depth
}


/// Calculate average of the vector
pub fn mean(data: & [u32]) -> f64{
    let sums1: u32 = data.iter().sum();
    let sums = (sums1 as f64)/data.iter().len() as f64;
    return sums
}

/// Calculate median of the vector
pub fn median(data: &mut Vec<u32>) -> u32{
    data.sort();
    return data[data.len()/2]
}

/// Get the file name
///
/// Remove folder structure
///
pub fn get_filename(name: &str) -> String{
    let u: Vec<&str> = name.split("/").collect();
    u.last().unwrap().to_string()


}

