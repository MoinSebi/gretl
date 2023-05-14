use clap::ArgMatches;
use crate::helpers::graphs::{make_wrapper, read_graph};
use crate::sliding_window::window::sw_wrapper;
use crate::sliding_window::writer::write_window;
use crate::stats::helper::get_filename;

/// Main function for node id to integer function
pub fn window_main(matches: &ArgMatches){
    let graph = read_graph(matches);
    let gw = make_wrapper(&graph, matches);
    let output = &get_filename(matches.value_of("output").unwrap());

    let mut window_size: u32 = 100000;
    if matches.is_present("window-size"){
        window_size = matches.value_of("window-size").unwrap().parse().unwrap();
    }


    let mut step: u32 = window_size;
    if matches.is_present("window-size"){
        step = matches.value_of("window-size").unwrap().parse().unwrap();
    }

    // similarity
    let metric = "similarity";

    let f = sw_wrapper(&graph, window_size, step);
    write_window(f, "hilfe");


}

