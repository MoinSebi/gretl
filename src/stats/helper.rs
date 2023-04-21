use std::collections::{HashMap, HashSet};
use gfa_reader::{Gfa, GraphWrapper};

/// Counting the amount
pub fn calculate_core(graph: &Gfa) -> HashMap<u32, (u32, u32)>{

    // Initialization hashmap
    let mut count: HashMap<u32, (u32, u32)> = HashMap::new();
    for x in &graph.nodes{
        count.insert(x.0.parse().unwrap(), (0, 0));
    }

    // Calculate the amount of sequence and the amount of nodes
    for x in &graph.paths{
        // Count every node only once
        let v: HashSet<_> = x.nodes.iter().cloned().collect();
        for y in v{
            count.get_mut(&y.parse().unwrap()).unwrap().1 += 1;
            count.get_mut(&y.parse().unwrap()).unwrap().0 += graph.nodes.get(&y).unwrap().len as u32
        }
    }
    count.shrink_to_fit();
    count
}

pub fn core1(graph: &Gfa) -> HashMap<u32, u32>{
    let mut count: HashMap<u32, u32> = HashMap::new();
    for x in &graph.nodes{
        count.insert(x.0.parse().unwrap(), 0);
    }

    for p in graph.paths.iter(){
        let v: HashSet<_> = p.nodes.iter().cloned().collect();
        for y in v.iter(){
            *count.get_mut(&y.parse().unwrap()).unwrap() += 1;
        }
    }
    count
}

pub fn core2(graph: &GraphWrapper, graph2: &Gfa) -> HashMap<u32, u32>{
    let mut count: HashMap<u32, u32> = HashMap::new();
    for x in &graph2.nodes{
        count.insert(x.0.parse().unwrap(), 0);
    }

    for p in graph.genomes.iter(){
        let mut v2: HashSet<_> = p.1[0].nodes.iter().cloned().collect();

        for x in p.1.iter(){
            let v: HashSet<_> = x.nodes.iter().cloned().collect();
            v2 = v2.union(&v).cloned().collect();

        }
        for y in v2.iter(){
            *count.get_mut(&y.parse().unwrap()).unwrap() += 1;
        }
    }
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


/// Calculate average of the vector
pub fn meanf(data: & [f32]) -> f64{
    let sums1: f32 = data.iter().sum();
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


/// Calculate node degree (in, out, total)
pub fn node_degree2(graph: &Gfa) -> (HashMap<&String, u32>, HashMap<&String, u32>, HashMap<&String, u32>){
    let mut degree_in: HashMap<&String, u32> = HashMap::new();
    let mut degree_out: HashMap<&String, u32> = HashMap::new();
    let mut degree_total: HashMap<&String, u32> = HashMap::new();
    for x in graph.edges.iter(){
        let fromu: u32 = x.from.parse().unwrap();
        let tou: u32 = x.to.parse().unwrap();

        if degree_in.contains_key(&x.from){
            degree_in.insert(&x.from, degree_in[&x.from]  +1 );
            degree_total.insert(&x.from, degree_total[&x.from]  +1 );

        } else {
            degree_in.insert(&x.from, 1);
            degree_total.insert(&x.from, 1 );
        }
        if degree_out.contains_key(&x.to){
            degree_out.insert(&x.to, degree_out[&x.to]  +1 );
            degree_total.insert(&x.to, degree_total[&x.to]  +1 );

        } else {
            degree_out.insert(&x.to, 1);
            degree_total.insert(&x.to, 1 );

        }
    }
    return (degree_in, degree_out, degree_total)

}
