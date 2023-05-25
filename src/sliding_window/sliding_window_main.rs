use clap::ArgMatches;
use crate::helpers::graphs::{make_wrapper, read_graph};
use crate::sliding_window::sliding_window_main::metric::nodesizem;
use crate::sliding_window::window::sliding_window_wrapper;
use crate::sliding_window::writer::write_window;
use crate::stats::helper::get_filename;

/// Main function for node id to integer function
pub fn window_main(matches: &ArgMatches){
    let graph = read_graph(matches);
    let gw = make_wrapper(&graph, matches);
    let output = matches.value_of("output").unwrap();

    let mut size: u32 = 100000;
    if matches.is_present("size"){
        size = matches.value_of("size").unwrap().parse().unwrap();
    }


    let mut step: u32 = size;
    if matches.is_present("step"){
        step = matches.value_of("step").unwrap().parse().unwrap();
    }

    let mut node = false;
    if matches.is_present("node"){
        node = true;
    }




    // similarity
    let mut metric = metric::similarity;
    if matches.is_present("metric"){
        match matches.value_of("metric").unwrap(){
            "metric" => metric = metric::nodesizem,
            "nodesize" => metric = metric::similarity,
            _ => metric = metric::similarity,
        }
    }

    let f = sliding_window_wrapper(&graph, size, step, metric , node);
    write_window(f, output);


}

pub enum metric{
    similarity,
    nodesizem
}

