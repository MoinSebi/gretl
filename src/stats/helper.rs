use std::collections::{HashMap, HashSet};
use gfa_reader::{Gfa, GraphWrapper, NCGfa, NCGraphWrapper};


pub fn calculate_core(graph: &NCGraphWrapper) -> Vec<u32>{
    let mut count: Vec<u32> = vec![0; graph.nodes.len()];

    for (name, p) in graph.genomes.iter(){
        let mut v2: HashSet<_> = p[0].nodes.iter().cloned().collect();
        for x in p.iter(){
            let v: HashSet<_> = x.nodes.iter().cloned().collect();
            v2 = v2.union(&v).cloned().collect();
        }
        for x in v2.iter(){
            for y in v.iter(){
                *count[*x as usize] += 1;
            }
        }
    }
    count
}

/// Counting the amount of accessions and depth
pub fn calculate_depth(graph: &NCGraphWrapper) -> Vec<u32>{
    let mut count: Vec<u32> = vec![0; graph.nodes.len()];
    for (name, p) in graph.genomes.iter() {
        for path in p.iter() {
            for x in path.nodes.iter() {
                *count[*x as usize] += 1;
            }
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


/// Calculate average of the vector
pub fn meanf(data: & [f32]) -> f64{
    let sums1: f32 = data.iter().sum();
    let sums = (sums1 as f64)/data.iter().len() as f64;
    return sums
}

/// Calculate median of the vector
pub fn median(data: &mut Vec<u32>) -> f64{
    data.sort();
    return data[data.len()/2] as f64
}

/// Get the file name
///
/// Remove folder structure
///
pub fn get_filename(name: &str) -> String{
    let u: Vec<&str> = name.split("/").collect();
    u.last().unwrap().to_string()


}


/// Calculate node degree (in, out, total)
pub fn node_degree(graph: &NCGraphWrapper, graph2: &NCGfa) -> (Vec<u32>, Vec<u32>, Vec<u32>){
    let mut degree_in: Vec<u32> = vec![0; graph.nodes.len()];
    let mut degree_out: Vec<u32> = vec![0; graph.nodes.len()];
    let mut degree_total: Vec<u32> = vec![0; graph.nodes.len()];
    for x in graph2.edges.iter(){
        let fromu: u32 = x.from.parse().unwrap();
        let tou: u32 = x.to.parse().unwrap();

        degree_out[fromu as usize] += 1;
        degree_in[tou as usize] += 1;
        degree_total[fromu as usize] += 1;
        degree_total[tou as usize] += 1;

    }
    return (degree_in, degree_out, degree_total)

}
