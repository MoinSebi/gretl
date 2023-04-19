use std::collections::{HashMap, HashSet};
use clap::{ArgMatches};
use gfa_reader::Gfa;
use crate::bootstrap::helper::random_numbers;
use crate::bootstrap::meta::make_meta;
use crate::bootstrap::writer::write_meta;

pub fn bootstrap_main(matches: &ArgMatches, graph: &Gfa){
    eprintln!("Running bootstrap");
    let amount_path = graph.paths.len();

    // Make a meta file
    if matches.is_present("meta"){
        eprintln!("Creating meta file");
        let meta = make_meta(graph, 10);
        write_meta(meta, "test2");

        // Run the bootstrap
    } else {
        // Reading the meta file
        if matches.is_present("input"){

        }
    }



    //let c: usize = matches.value_of("number").unwrap().parse().unwrap();

    let mut total = Vec::new();
    if true{
        for x in 2..amount_path+1{
            let test_comb = combinations_maker(&6, &x, &5);
            let mut real_res = Vec::new();
            for (i, x1) in test_comb.iter().enumerate(){
                println!("hit");
                let k: Vec<usize> = x1.iter().cloned().collect();
                let dd = do_one_iteration(graph, &k);
                println!("{} {:?} {:?}", x, i, dd);
                real_res.push((x, i, dd));
            }
            total.push(real_res);
        }
    }
    //do_one_iteration(graph, &test_comb[0]);







}


///
///
/// - Total size
/// - Number to sample
/// - Who many
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



pub fn do_one_iteration(graph: &Gfa, combination: &[usize]) -> Vec<usize>{
    let count = calculate_core(graph, combination);
    let f = combination.len();
    let mut result: Vec<usize> = vec![0; f+1];
    let mut result2 = vec![0; f+1];
    graph.nodes.iter().for_each(|n|
        {let id = n.0.parse::<usize>().unwrap();
            let id = count.get(&(id as u32)).unwrap();
            result[*id as usize] += 1;
            result2[*id as usize] += graph.nodes.get(n.0).unwrap().len});
    return result

}


/// Counting the amount of accessions and depth
pub fn calculate_core(graph: &Gfa, combination: &[usize]) -> HashMap<u32, u32>{
    let mut count: HashMap<u32, u32> = HashMap::new();
    for x in &graph.nodes{
        count.insert(x.0.parse().unwrap(), 0);
    }

    for (i, x) in graph.paths.iter().enumerate(){
        if combination.contains(&i){
            let v: HashSet<_> = x.nodes.iter().cloned().collect();
            for y in v{
                *count.get_mut(&y.parse().unwrap()).unwrap() += 1;
            }
        }
    }
    count.shrink_to_fit();
    count
}