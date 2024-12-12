use crate::nwindow::n_windows::nwindow_wrapper;
use crate::nwindow::writer_nwindow::write_list;
use clap::ArgMatches;
use gfa_reader::{check_numeric_compact_gfafile, Gfa};
use log::{info, warn};
use std::process;

pub fn nwindow_main(matches: &ArgMatches) {
    info!("Running 'gretl nwindow'");

    let mut window_nodes = u32::MAX;
    let mut window_size = u32::MAX;
    let mut window_jumps = u32::MAX;

    if matches.is_present("step") {
        window_nodes = matches.value_of("step").unwrap().parse().unwrap();
    }
    if matches.is_present("sequence") {
        window_size = matches.value_of("sequence").unwrap().parse().unwrap();
    }
    if matches.is_present("jump") {
        window_jumps = matches.value_of("jump").unwrap().parse().unwrap();
    }
    if window_nodes == u32::MAX && window_size == u32::MAX && window_jumps == u32::MAX {
        window_nodes = 10;
    }

    let threads = matches.value_of("threads").unwrap().parse().unwrap();
    let output = matches.value_of("output").unwrap();

    info!("GFA file: {}", matches.value_of("gfa").unwrap());
    info!(
        "Window nodes: {}",
        if window_nodes == u32::MAX {
            "None".to_string()
        } else {
            window_nodes.to_string()
        }
    );
    info!(
        "Window size: {}",
        if window_size == u32::MAX {
            "None".to_string()
        } else {
            window_size.to_string()
        }
    );
    info!(
        "Window jump: {}",
        if window_jumps == u32::MAX {
            "None".to_string()
        } else {
            window_jumps.to_string()
        }
    );
    info!(
        "Output file: {}",
        if output == "-" { "stdout" } else { output }
    );
    info!("Threads: {}", threads);

    info!("Numeric check");
    let (numeric, sorted) = check_numeric_compact_gfafile(matches.value_of("gfa").unwrap());
    if numeric {
        if !sorted {
            warn!("The GFA file is not sorted. All 'jump' stats might be without sense.")
        }
        // Read the graph

        info!("Read GFA file");
        let graph: Gfa<u32, (), ()> =
            Gfa::parse_gfa_file_multi(matches.value_of("gfa").unwrap(), threads);

        if window_nodes == u32::MAX && window_size == u32::MAX && window_jumps == u32::MAX {
            warn!(
                "No window size, step or jump defined. Using default value - node window size: 10"
            );
        }

        info!("Run nwindow core algorithm");
        let a = nwindow_wrapper(
            &graph,
            window_nodes,
            window_size,
            window_jumps as u128,
            "all",
            threads,
        );

        info!(
            "Writing to file: {}",
            if output == "-" { "stdout" } else { output }
        );
        write_list(&a, output, &graph.segments);
        info!("Done");
    } else {
        panic!("The node IDs in the GFA file are not numeric");
    }
}
