mod stats;

use crate::stats::{mean_median_graph_size, input_genomes, node_degree, inverted_edges, edges_nodes_number};
use std::collections::HashMap;
use std::env;
use argparse::{ArgumentParser, StoreOption, StoreTrue, Store};


fn printing(results: HashMap<&str, f32>){
    for (k,_v) in results.iter(){
        print!("{}\t", k);
    }
    print!("\n");
    for (_k,v) in results.iter(){
        if v.fract() == 0.0 {
            print!("{:.4}\t", *v as u32);
        }
        else {
            print!("{:.4}\t", v);
        }
    }
    print!("\n");
}

fn combine(results: Vec<HashMap<&str, f32>>) -> HashMap<&str, f32>{
    let mut combined_hm: HashMap<&str, f32> = HashMap::new();
    for x in results.iter(){
        for (k,v) in x.iter(){
            combined_hm.insert(k, v.clone());
        }
    }
    combined_hm
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

    println!("These are the command line arguments {:?}", name);
    // Read the graph
    let graph = gfaR::readGFA(&name);

    let mut stats: Vec<HashMap<&str, f32>> = Vec::new();

    stats.push(mean_median_graph_size(&graph));
    stats.push(input_genomes(&graph));
    stats.push(node_degree(&graph));
    stats.push(inverted_edges(&graph));
    stats.push(edges_nodes_number(&graph));

    let combined = combine(stats);
    printing(combined);
}
