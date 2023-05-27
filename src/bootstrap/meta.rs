use std::collections::{HashMap, HashSet};
use gfa_reader::{Gfa, GraphWrapper};
use crate::bootstrap::helper::random_numbers;

/// Wrapper for combinations
///
/// Creates meta data for the bootstrap
/// One entry consists of: [number of genomes, number of iteration, combination (HashSet)]
pub fn combinations_maker_wrapper(size: &usize, amount: &usize) -> Vec<(usize, usize, HashSet<usize>)> {
    let mut data = vec![];
    for x in 2..size+1 {
        let test_comb = combinations_maker(&size, &x, amount);
        for (i, x1) in test_comb.iter().enumerate(){
            data.push((x,i, x1.clone()))
        }
    }
    data
}


/// Makes multiple random combinations
/// size = total sample size [0,1,2,3,4,5,6,7,8,9]
/// number = size of the sample (3 -> [1,2,3])
/// amount = amount og samples that should be drawn (2 -> [[1,2,3], [4,5,6]])
///
/// TODO
/// Make a check when not more different samples can be drawn (very unlikely)
pub fn combinations_maker(size: &usize, core_number: &usize, amount: &usize) -> Vec<HashSet<usize>>{
    let mut result = Vec::new();
    let mut counter = 0;
    while result.len() != *amount {
        let v = random_numbers(size, core_number);
        let v: HashSet<usize> = v.iter().cloned().collect();
        if ! result.contains(&v){
            result.push(v);
        }
        if counter > 10000{
            return result
        }
        counter += 1;
    }
    return result
}

/// Removes lines and unused similarity level from the meta data (file)
pub fn reduce_meta(meta: &mut Vec<(usize, usize, HashSet<usize>)>, line: i32, core: i32){
    if line != -1{
        let value_to_retain = meta[line as usize].clone();
        meta.retain(|x| *x == value_to_retain );


    } else {
        if core != -1 {
            meta.retain(|x| x.0 == core as usize);
        }
    }
}



/// Calculation for one iteration
///
/// TODO:
/// - more metrics
pub fn one_iteration(gw: &GraphWrapper, graph: &Gfa, combination: &[usize], metric: &str) -> (Vec<usize>, Vec<usize>){
    let mut metric_hm: HashMap<u32, u32> = calculate_core_reduced(gw, graph, combination);


    // Function to get the amount unique values in u32, u32 HashMap (metric_hm)
    let f = metric_hm.values().collect::<HashSet<_>>().len();
    let maxf = *metric_hm.values().collect::<HashSet<_>>().iter().max().unwrap().clone();

    // Saving results as number of nodes and amount of sequence
    let mut result: Vec<usize> = vec![0; maxf as usize +1];             // NODES
    let mut result2 = vec![0; maxf as usize +1];             // Sequence

    // Iterate over all nodes and add the amount of nodes and sequence to the result vector
    graph.nodes.iter().for_each(|n|
        {let id = n.0.parse::<usize>().unwrap();
            let id = metric_hm.get(&(id as u32)).unwrap();
            result[*id as usize] += 1;
            result2[*id as usize] += graph.nodes.get(n.0).unwrap().len});

    // Remove first index (0) from the result vector
    result.remove(0);
    result2.remove(0);

    return (result, result2)

}

/// Calculate similarity level and ignore path which are not in the combination set
pub fn calculate_core_reduced(graph2: &GraphWrapper, graph: &Gfa, combination: &[usize]) -> HashMap<u32, u32>{
    let mut count: HashMap<u32, u32> = HashMap::new();
    for x in &graph.nodes{
        count.insert(x.0.parse().unwrap(), 0);
    }

    for (i, x) in graph2.genomes.iter().enumerate(){
        if combination.contains(&i){
            let mut vv: HashSet<String> = HashSet::new();
            for y in x.1.iter(){
                let v: HashSet<_> = y.nodes.iter().cloned().collect();
                vv.extend(v);
            }

            for y in vv{
                *count.get_mut(&y.parse().unwrap()).unwrap() += 1;
            }
        }
    }
    count.shrink_to_fit();
    count
}

