use std::cmp::{min};
use std::collections::{HashSet};
use clap::{ArgMatches};
use gfa_reader::{GraphWrapper, NCGfa, NCPath};
use crate::bootstrap::meta::{combinations_maker_wrapper, one_iteration, reduce_meta};
use crate::bootstrap::reader::read_meta;
use crate::bootstrap::writer::{write_meta, write_output};
use crate::helpers::helper::calculate_similarity;
use rayon::prelude::*;



/// Main function for bootstrapping
pub fn bootstrap_main(matches: &ArgMatches){

    let mut sep = " ";
    if matches.is_present("Pan-SN"){
        sep = matches.value_of("Pan-SN").unwrap();
    }

    let threads = matches.value_of("Threads").unwrap().parse::<usize>().unwrap();


    // Read the graph
    let mut graph: NCGfa<()> = NCGfa::new();
    graph.parse_gfa_file_and_convert(matches.value_of("gfa").unwrap(), false);
    let mut wrapper: GraphWrapper<NCPath> = GraphWrapper::new();



    wrapper.from_gfa(&graph.paths, sep);
    let output = matches.value_of("output").unwrap();

    // Get the amount of iterations
    let mut amount = 10;
    if matches.is_present("number"){
        amount = matches.value_of("number").unwrap().parse().unwrap();
    }

    // Limit the amount of iterations (maximum 500)
    amount = min(amount, 500);

    // Combination: {number of genomes, number of iteration, combination (HashSet)}
    let mut combinations: Vec<(usize, usize, HashSet<usize>)>;
    if matches.is_present("meta input"){
        combinations = read_meta(matches.value_of("meta input").unwrap());
    } else {
        combinations = combinations_maker_wrapper(&wrapper.genomes.len(), &amount);
    }

    // Which line should be read
    let mut line = -1;
    if matches.is_present("meta line"){
        line = matches.value_of("meta line").unwrap().parse().unwrap();
    }

    // Which core should be used
    let mut core = -1;
    if matches.is_present("level"){
        core = matches.value_of("level").unwrap().parse().unwrap();
    }

    eprintln!("Running bootstrap");

    // The which "geomes" have been used in this run
    // let mut metas = Vec::new();

    // How much sequence, nodes have been used
    // let mut total = Vec::new();

    // Removes lines and unused similarity level from the meta data (file)
    reduce_meta(& mut combinations, line, core);

    // We use the similarity measure
    let similarity = calculate_similarity(&&wrapper, &graph);

    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .unwrap();

    let results: Vec<_> = thread_pool.install(|| {
        combinations.par_chunks(5) // Process in chunks of 3 elements (you can adjust the chunk size).
            .flat_map(|chunk| {
                chunk.into_iter().map(|(number_genomes, iterations, combination)| {
                    let combi: Vec<usize> = combination.iter().cloned().collect();
                    let result_one_iteration = one_iteration(&wrapper, &graph, &combi, "similarity", &similarity);

                    // Return results without a semicolon
                    (*number_genomes, *iterations, result_one_iteration, combination)
                }).into_iter().collect::<Vec<_>>()
            }).into_par_iter() // Pass external data to the processing function.
            .collect()

    });

    let mut metas = Vec::new();
    for x in results.iter(){
        metas.push((x.0, x.1, x.3.clone()));
    }

    let mut total = Vec::new();
    for x in results.iter(){
        total.push((x.0, x.1, x.2.clone()));
    }

    // // Iterate over all combinations - calculate the core and the sequence
    // for (number_genomes, iterations, combination) in combinations.iter(){
    //     let combi: Vec<usize> = combination.iter().cloned().collect();
    //     let result_one_iteration = one_iteration(&wrapper, &graph, &combi, "similarity", &similarity);
    //
    //     // Add results
    //     total.push((*number_genomes, *iterations, result_one_iteration));
    //     metas.push((*number_genomes, *iterations, combination.clone()));
    // }

    // Write the meta data if wanted
    if matches.is_present("meta"){
        let metas_output = matches.value_of("meta").unwrap();
        write_meta(metas, metas_output);
    }

    // Write the main output
    write_output(total, output);

}


