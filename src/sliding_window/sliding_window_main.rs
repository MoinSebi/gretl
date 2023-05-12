use clap::ArgMatches;
use crate::helpers::graphs::{make_wrapper, read_graph};
use crate::stats::helper::get_filename;

/// Main function for node id to integer function
pub fn window_main(matches: &ArgMatches){
    let graph = read_graph(matches);
    let gw = make_wrapper(&graph, matches);
    let output = &get_filename(matches.value_of("output").unwrap());

    let mut window_size = 100000;
    let mut step = 100000;
    let metric = "similarity";



}

