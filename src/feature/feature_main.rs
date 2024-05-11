use crate::helpers::helper::{calc_depth, calc_node_degree, calc_node_len};
use clap::ArgMatches;
use gfa_reader::{Gfa, Pansn};
use std::fs::File;
use std::io::{BufWriter, Write};
use log::info;
use nalgebra::inf;


/// Feature main function
pub fn feature_main(matches: &ArgMatches) {
    // input, output
    let graph_file = matches.value_of("gfa").unwrap();
    let file_output = matches.value_of("output").unwrap();

    let mut maxlen: i128 = matches.value_of("max-len").unwrap_or("-9").parse().unwrap();
    let minlen: i128 = matches.value_of("min-len").unwrap_or("-9").parse().unwrap();
    let mut maxdegree: i128 = matches
        .value_of("max-depth")
        .unwrap_or("-9")
        .parse()
        .unwrap();
    let mindegree: i128 = matches
        .value_of("min-depth")
        .unwrap_or("-9")
        .parse()
        .unwrap();
    let mut maxdepth: i128 = matches
        .value_of("max-depth")
        .unwrap_or("-9")
        .parse()
        .unwrap();
    let mindepth: i128 = matches
        .value_of("min-depth")
        .unwrap_or("-9")
        .parse()
        .unwrap();
    let pansn_sep = matches.value_of("PanSN").unwrap_or(" ");

    info!("Running feature filter");


    // Read the graph and make wrapper
    let mut graph: Gfa<u32, (), ()> = Gfa::parse_gfa_file(graph_file);
    graph.walk_to_path(pansn_sep);
    let wrapper: Pansn<u32, (), ()> = Pansn::from_graph(&graph.paths, pansn_sep);

    if maxdegree == maxdepth
        && maxdegree == maxlen
        && maxdegree == -9
        && mindepth == maxdegree
        && mindegree == maxdegree
        && minlen == maxdegree
    {
        return;
    } else {
        if maxlen == -9 {
            maxlen = i128::MAX;
        }
        if maxdepth == -9 {
            maxdepth = i128::MAX;
        }
        if maxdegree == -9 {
            maxdegree = i128::MAX;
        }
    }

    info!("Graph file: {}", graph_file);
    info!("Output file: {}", file_output);
    info!("Max length: {}", maxlen);
    info!("Min length: {}", minlen);
    info!("Max degree: {}", maxdegree);
    info!("Min degree: {}", mindegree);
    info!("Max depth: {}", maxdepth);
    info!("Min depth: {}", mindepth);
    info!("PanSN separator: {}", pansn_sep);

    // Run filter
    let result = feature_filter(
        &graph, &wrapper, mindepth, mindegree, minlen, maxlen, maxdegree, maxdepth,
    );

    info!("Write output");
    // Write output
    write_list(&result, file_output);
}


/// Filter feature by length, degree and depth
pub fn feature_filter(
    graph: &Gfa<u32, (), ()>,
    wrapper: &Pansn<u32, (), ()>,
    mindepth: i128,
    mindegree: i128,
    minlen: i128,
    maxlen: i128,
    maxdegree: i128,
    maxdepth: i128,
) -> Vec<usize> {
    let paths = wrapper.get_path_genome();

    let mut result = Vec::new();
    let size = calc_node_len(graph);
    let degree = calc_node_degree(graph).2;
    let depth = calc_depth(&paths, graph);
    for (i, (s, (deg, dep))) in size
        .iter()
        .zip(degree.iter().zip(depth.iter()))
        .enumerate()
    {
        if s != &0 {
            if *s as i128 > minlen
                && *deg as i128 > mindegree
                && *dep as i128 > mindepth
                && (*s as i128) < maxlen
                && (*deg as i128) < maxdegree
                && (*dep as i128) < maxdepth
            {
                result.push(i)
            }
        }
    }
    result
}

/// Write the nodes to a file
pub fn write_list(data: &Vec<usize>, filename: &str) {
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);

    for x in data.iter() {
        writeln!(f, "{}", x).expect("Write list error");
    }
}

