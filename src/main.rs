mod graph_stats;
mod path;
mod helper;
mod writer;

use clap::{Arg, App, AppSettings};
use gfa_reader::Gfa;
use crate::graph_stats::graph_stats_wrapper;
use crate::helper::get_filename;
use crate::path::path_stats_wrapper;
use crate::writer::{write_tsv_path, write_yaml, write_yaml_path};

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
    let filename = get_filename(&gfa);
    eprintln!("The filename is {}", filename);

    if matches.is_present("path"){
        let data = path_stats_wrapper(&graph);
        let tab = ["Seq len",
            "Node length (seq)",
            "Nodes length (node)",
            "Unique nodes",
            "Jumps total",
            "Jumps ratio",
            "Jumps bigger than ",
            "Average depth",
            "Median depth",
            "Average similarity",
            "Median similarity"];
        write_tsv_path(&data, &tab, "test");
    } else {
        let data = graph_stats_wrapper(&graph);
        let tab = ["#Path",
        "#Nodes",
        "#Edges",
        "Node length [average]",
        "Node length [mediant]",
        "Node length [total]",
        "Input genomes [total]",
        "Graph degree [in]",
        "Graph degree [out]",
        "Graph degree [total]"];
        write_yaml(&data, &tab, "test");
    }

}
