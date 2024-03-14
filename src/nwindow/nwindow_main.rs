use crate::nwindow::n_windows::stats2;
use crate::nwindow::writer_nwindow::{make_buffer, write_list};
use clap::ArgMatches;
use gfa_reader::NCGfa;
use log::info;

pub fn nwindow_main(matches: &ArgMatches) {
    let mut window_nodes = u32::MAX;
    let mut window_size = u32::MAX;
    let mut window_metric = u32::MAX;

    if matches.is_present("nodes") {
        window_nodes = matches.value_of("nodes").unwrap().parse().unwrap();
    }
    if matches.is_present("size") {
        window_size = matches.value_of("size").unwrap().parse().unwrap();
    }
    if matches.is_present("metric") {
        window_metric = matches.value_of("metric").unwrap().parse().unwrap();
    }
    if window_nodes == u32::MAX || window_size == u32::MAX || window_metric == u32::MAX {
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

    let mut graph: NCGfa<()> = NCGfa::new();
    graph.parse_gfa_file_and_convert(matches.value_of("gfa").unwrap(), true);

    let output = matches.value_of("output").unwrap();

    let a = stats2(
        &graph,
        window_nodes,
        window_size,
        window_metric as u128,
        rtype,
    );

    info!("Writing to file: {}", output);
    let mut ff = make_buffer(output);
    write_list(&a, &mut ff);
}
