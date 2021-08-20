mod stats;

use crate::stats::{mean_median_graph_size, input_genomes, node_degree, inverted_edges, edges_nodes_number};
use std::collections::HashMap;
use std::env;
use argparse::{ArgumentParser, StoreOption, StoreTrue, Store};
use std::path::Path;



fn printing(results: HashMap<&str, String>){
    for (k,_v) in results.iter(){
        print!("{}\t", k);
    }
    print!("\n");
    for (_k,v) in results.iter(){
        print!("{}\t", v);
    }
    print!("\n");
}

fn combine(results: Vec<HashMap<&str, String>>) -> HashMap<&str, String>{
    let mut combined_hm: HashMap<&str, String> = HashMap::new();
    for x in results.iter(){
        for (k,v) in x.iter(){
            combined_hm.insert(k, v.clone());
        }
    }
    combined_hm
}

fn get_filename(name: &String) -> HashMap<&str, String>{
    let u: Vec<&str> = name.split("/").collect();
    let mut result: HashMap<&str, String> = HashMap::new();

    result.insert("File name" , u.last().unwrap().to_string());
    result

}

fn main() {
    let mut name = "World".to_string();
    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("GFA statistics");
        ap.refer(&mut name)
            .add_option(&["-i", "--input"], Store,
                        "Input data").required();
        ap.parse_args_or_exit();
    }

    // Read the graph
    eprintln!("File name: {}", &name);

    let graph = gfaR::readGFA(&name);

    let mut stats: Vec<HashMap<&str, String>> = Vec::new();


    stats.push(get_filename(&name));
    stats.push(mean_median_graph_size(&graph));
    stats.push(input_genomes(&graph));
    stats.push(node_degree(&graph));
    stats.push(inverted_edges(&graph));
    stats.push(edges_nodes_number(&graph));



    let combined = combine(stats);
    printing(combined);
}
