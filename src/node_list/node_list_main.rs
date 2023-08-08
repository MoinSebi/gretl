use clap::ArgMatches;
use gfa_reader::{Gfa, GraphWrapper, NCGfa, NCPath};
use crate::node_list::wrapper::wrapper_node;
use crate::stats::helper::get_filename;


/// Main function for node list
pub fn nodelist_main(matches: &ArgMatches) {
    eprintln!("Running node-list analysis.");
    // Size, depth, similarity, degree in, degree out, degree_total, inversion amount,
    // path related?
    let mut graph: NCGfa<()> = NCGfa::new();
    graph.parse_gfa_file_and_convert(matches.value_of("gfa").unwrap(), true);
    let mut wrapper: GraphWrapper<NCPath> = GraphWrapper::new();
    wrapper.from_gfa(&graph.paths, " ");

    let output = matches.value_of("output").unwrap();
    let splits = vec!["Core", "Length", "Depth", "Core", "ND"];
    let mut splits2 = Vec::new();
    if matches.is_present("Features"){
        splits2 = matches.value_of("Features").unwrap().split(",").collect();
    }
    let mut final1 = Vec::new();
    for x in splits2.iter(){
        if splits.contains(x) {
            final1.push(*x);
        }
    }

    if final1.len() == 0{
        final1 = splits.clone();
    }
    // This wrapper also writes data to a file
    wrapper_node(&graph, &wrapper, output, final1);


}