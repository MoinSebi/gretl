use crate::bootstrap::helper::read_values_from_file;
use crate::bootstrap::meta::{combinations_maker_wrapper, index1, one_iteration};

use crate::bootstrap::writer::{write_meta, write_output};
use crate::helpers::helper::calc_similarity;

use clap::ArgMatches;
use gfa_reader::{check_numeric_gfafile, Gfa, Pansn};
use log::info;
use rayon::prelude::*;

use chrono::Local;
use std::collections::HashSet;
use std::io::Write;
use std::sync::atomic::{AtomicU32, Ordering};

/// Main function for bootstrapping
pub fn bootstrap_main(matches: &ArgMatches) {
    info!("Running 'gretl bootstrap' analysis");
    let mut sep = matches.value_of("Pan-SN").unwrap();

    let threads = matches
        .value_of("threads")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let gfa_file = matches.value_of("gfa").unwrap();
    let output = matches.value_of("output").unwrap();
    let amount = matches
        .value_of("number")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    info!("Gfa file: {}", matches.value_of("gfa").unwrap());
    info!("Pan-SN: {}", if sep == "\n" { "None" } else { sep });
    info!(
        "Output file: {}",
        if output == "-" { "stdout" } else { output }
    );
    info!("Threads: {}", threads);

    if check_numeric_gfafile(matches.value_of("gfa").unwrap()) {
        // Read the graph
        info!("Read the graph");
        let mut graph: Gfa<u32, (), ()> = Gfa::parse_gfa_file_multi(gfa_file, threads);
        if graph.paths.is_empty() && sep == "\n" {
            sep = "#"
        }
        graph.walk_to_path(sep);
        if graph.paths.is_empty() {
            panic!("Error: No path found in graph file");
        }

        let wrapper: Pansn<u32, (), ()> = Pansn::from_graph(&graph.paths, sep);
        let mut nodes: HashSet<_> = graph.segments.iter().map(|n| n.id).collect();
        if matches.is_present("nodes") {
            let a = read_values_from_file(matches.value_of("nodes").unwrap());
            nodes = a.iter().cloned().collect();
        }

        info!("Create all the combinations");
        // Combination: {number of genomes, number of iteration, combination (HashSet)}
        let combinations: Vec<(usize, usize, HashSet<usize>)> =
            combinations_maker_wrapper(&wrapper.genomes.len(), &amount);

        let paths = wrapper.get_path_genome();

        // We use the similarity measure
        info!("Calculate the similarity");
        let similarity = calc_similarity(&paths, &graph);
        info!("Similarity done");
        let max_val = similarity.len();
        let max_sim = *similarity.iter().max().unwrap() as usize;

        info!("Number of runs: {:?}", combinations.len());
        let o = wrapper.get_path_genome();
        let index = index1(&o, threads);

        let chunk_size = (combinations.len() + threads - 1) / threads;
        let counter = AtomicU32::new(0);

        info!("Start the parallel computation");
        let results: Vec<_> = combinations
            .par_chunks(chunk_size) // Process in chunks of 5 elements (you can adjust the chunk size).
            .flat_map(|chunk| {
                chunk
                    .iter()
                    .map(|(number_genomes, iterations, combination)| {
                        let combi: Vec<usize> = combination.iter().cloned().collect();
                        let result_one_iteration = one_iteration(
                            &wrapper,
                            &graph,
                            &combi,
                            &similarity,
                            &nodes,
                            max_val as usize,
                            max_sim,
                            &index,
                        );
                        counter.fetch_add(1, Ordering::Relaxed);
                        std::io::stderr().flush().unwrap();
                        eprint!(
                            "\r{}          Bootstrap {:?}/{}",
                            Local::now().format("%d/%m/%Y %H:%M:%S %p"),
                            counter.load(Ordering::Relaxed),
                            combinations.len()
                        );

                        // Return results without a semicolon
                        (
                            *number_genomes,
                            *iterations,
                            combination,
                            result_one_iteration,
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        eprintln!();

        // Write the meta data if wanted
        if matches.is_present("meta-output") {
            info!("Write the meta data");
            let metas_output = matches.value_of("meta-output").unwrap();
            write_meta(&results, metas_output);
        }

        // Write the output
        info!("Write the output");
        write_output(&results, output);
    } else {
        panic!("GFA file is not numeric");
    }
}
