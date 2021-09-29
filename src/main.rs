mod stats;

use crate::stats::{mean_median_graph_size, input_genomes, node_degree, inverted_edges, edges_nodes_number};
use argparse::{ArgumentParser, Store};
use gfaR::Gfa;


/// Printing the stats
///
fn printing(results: Vec<(&str, String)>){
    for (k,_v) in results.iter(){
        print!("{}\t", k);
    }
    print!("\n");
    for (_k,v) in results.iter(){
        print!("{}\t", v);
    }
    print!("\n");
}

/// Combine multiple vectors into one
fn combine(results: Vec<Vec<(&str, String)>>) -> Vec<(&str, String)>{
    let mut combined_vector: Vec<(&str, String)> = Vec::new();
    for x in results.iter(){
        for (k, v) in x.iter(){
            combined_vector.push((k, v.clone()));
        }
    }
    combined_vector
}

/// Get the file name
///
/// Remove folder structure
///
fn get_filename(name: &String) -> Vec<(&str, String)>{
    let u: Vec<&str> = name.split("/").collect();
    let mut result: Vec<(&str, String)> = Vec::new();

    result.push(("File name" , u.last().unwrap().to_string()));
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

    let mut graph = Gfa::new();
    graph.read_file(&name);

    let mut stats: Vec<Vec<(&str, String)>> = Vec::new();

    stats.push(get_filename(&name));
    stats.push(edges_nodes_number(&graph));
    stats.push(mean_median_graph_size(&graph));
    stats.push(input_genomes(&graph));
    stats.push(node_degree(&graph));
    stats.push(inverted_edges(&graph));


    let combined = combine(stats);

    printing(combined);
}
