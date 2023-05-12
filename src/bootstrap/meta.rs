use std::collections::{HashMap, HashSet};
use gfa_reader::{Gfa, GraphWrapper};
use crate::bootstrap::helper::random_numbers;

/// Make a meta file
/// It's a table (tab-sep)
/// -core
/// -run
/// -combination(comma sep)
///
///
pub fn make_meta(graph: &Gfa, amount: usize) -> Vec<(usize, usize, HashSet<usize>)>{
    let mut f = Vec::new();

    for y in 2..graph.paths.len()+1 {
        let test_comb = combinations_maker(&graph.paths.len(), &y, &amount);
        test_comb.iter().for_each(|n| f.push((graph.paths.len(), y, n.clone())))
    }

    return f
}

pub fn combinations_maker_wrapper(size: &usize, amount: &usize) -> Vec<(usize, usize, HashSet<usize>)> {
    let mut data = vec![];
    for x in 2..size+1 {
        let test_comb = combinations_maker(&4, &x, &2);
        for (i, x1) in test_comb.iter().enumerate(){
            data.push((x,i, x1.clone()))
        }
    }
    data
}


/// Makes multiple random combinations
/// size = wie groß (raw)
/// number = wieviel samples (anzahl)
/// amount = wie oft (bootstrap)
pub fn combinations_maker(size: &usize, core_number: &usize, amount: &usize) -> Vec<HashSet<usize>>{
    let mut res = Vec::new();
    let mut c = 0;
    while res.len() != *amount {
        let v = random_numbers(size, core_number);
        let v: HashSet<usize> = v.iter().cloned().collect();
        if ! res.contains(&v){
            res.push(v);
        }
        if c > 10000{
            return res
        }
        c += 1;
    }
    return res

}




/// Calculation for one iteration
pub fn do_one_iteration2(gw: &GraphWrapper, graph: &Gfa, combination: &[usize]) -> (Vec<usize>, Vec<usize>){
    let count = calculate_core2(gw, graph, combination);
    let f = combination.len();
    let mut result: Vec<usize> = vec![0; f+1];             // NODES
    let mut result2 = vec![0; f+1];             // Sequence
    graph.nodes.iter().for_each(|n|
        {let id = n.0.parse::<usize>().unwrap();
            let id = count.get(&(id as u32)).unwrap();
            result[*id as usize] += 1;
            result2[*id as usize] += graph.nodes.get(n.0).unwrap().len});
    result.remove(0);
    result2.remove(0);
    return (result, result2)

}

/// Counting the amount of accessions and depth
pub fn calculate_core2(graph2: &GraphWrapper, graph: &Gfa, combination: &[usize]) -> HashMap<u32, u32>{
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