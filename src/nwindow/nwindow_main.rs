use std::process;
use crate::nwindow::n_windows::stats2;
use crate::nwindow::writer_nwindow::{make_buffer, write_list};
use clap::ArgMatches;
use gfa_reader::{check_numeric_gfafile, Gfa};
use log::info;

pub fn nwindow_main(matches: &ArgMatches) {
    info!("Running 'gretl nwindow'");

    let mut window_nodes = u32::MAX;
    let mut window_size = u32::MAX;
    let mut window_metric = u32::MAX;

    if matches.is_present("step") {
        window_nodes = matches.value_of("step").unwrap().parse().unwrap();
    }
    if matches.is_present("sequence") {
        window_size = matches.value_of("sequence").unwrap().parse().unwrap();
    }
    if matches.is_present("jump") {
        window_metric = matches.value_of("jump").unwrap().parse().unwrap();
    }
    if window_nodes == u32::MAX && window_size == u32::MAX && window_metric == u32::MAX {
        eprintln!("No window criteria provided. Default node: 10");
        window_nodes = 10;
    }

    let sum_nodes = matches.is_present("number of nodes");
    let sum_length = matches.is_present("sequence length");
    let sum_jumps = matches.is_present("sum-of-jumps");
    let mut rtype = "all";

    if sum_nodes || sum_length || sum_jumps {
        if sum_nodes {
            rtype = "nodes";
        }
        if sum_length {
            rtype = "sequence";
        }
        if sum_jumps {
            rtype = "jumps";
        }
    }

    info!("Gfa file: {}", matches.value_of("gfa").unwrap());
    info!("Output file: {}", matches.value_of("output").unwrap());
    info!("Window nodes: {}", window_nodes);
    info!("Window size: {}", window_size);
    info!("Window metric: {}", window_metric);
    info!("Sum nodes: {}", sum_nodes);
    info!("Sum length: {}", sum_length);
    info!("Sum jumps: {}", sum_jumps);
    info!("Return type: {}", rtype);


    if check_numeric_gfafile(matches.value_of("gfa").unwrap()) {
        // Read the graph
        let graph: Gfa<u32, (), ()> = Gfa::parse_gfa_file(matches.value_of("gfa").unwrap());
        let output = matches.value_of("output").unwrap();

        let a = stats2(
            &graph,
            window_nodes,
            window_size,
            window_metric as u128,
            rtype,
        );

        info!("Writing to file: {}", output);
        let mut buffer = make_buffer(output);
        write_list(&a, &mut buffer, &graph.segments);
    } else {
        eprintln!("GFA file is not numeric");
        process::exit(1);
    }
}
