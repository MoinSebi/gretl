mod stats;
mod path;
mod helper;
mod writer;

use crate::stats::{mean_median_graph_size, input_genomes, node_degree, inverted_edges, edges_nodes_number, single_paths};
use clap::{Arg, App, AppSettings};
use gfa_reader::Gfa;
use crate::path::path_stats_wrapper;

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
fn get_filename(name: &str) -> Vec<(&str, String)>{
    let u: Vec<&str> = name.split("/").collect();
    let mut result: Vec<(&str, String)> = Vec::new();

    result.push(("File name" , u.last().unwrap().to_string()));
    result

}



fn main() {


    let matches = App::new("gfastats")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.1.0")
        .author("Sebastian V")
        .about("GFa stats")

        .help_heading("Input options")
        .arg(Arg::new("gfa")
            .short('g')
            .long("gfa")
            .about("Input GFA file")
            .takes_value(true)
            .required(true))
        .arg(Arg::new("structure")
            .short('s')
            .long("structure")
            .about("Statistics based on structure of the graph"))
        .arg(Arg::new("path")
            .short('p')
            .long("path")
            .about("Path based structure"))

        .help_heading("Output options")
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .about("Output"))
        .arg(Arg::new("tsv")
            .short('t')
            .about("Tab seperated values format "))
        .arg(Arg::new("YAML")
            .short('y')
            .about("yaml format"))

        .get_matches();

    // Read the graph
    let gfa = matches.value_of("gfa").unwrap();
    let mut graph = Gfa::new();
    graph.read_file(&gfa);
    eprintln!("File name: {}", gfa);
    if matches.is_present("path"){
        path_stats_wrapper(&graph);
    } else {

    }

    let mut stats: Vec<Vec<(&str, String)>> = Vec::new();

    stats.push(get_filename(&gfa));
    stats.push(edges_nodes_number(&graph));
    stats.push(mean_median_graph_size(&graph));
    stats.push(input_genomes(&graph));
    stats.push(node_degree(&graph));
    stats.push(inverted_edges(&graph));
    stats.push(single_paths(&graph));


    let combined = combine(stats);

    printing(combined);
}
