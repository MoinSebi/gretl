use crate::path::writer::write_paths;
use crate::stats::path_stats::path_stats_wrapper;
use clap::ArgMatches;
use gfa_reader::{check_numeric_gfafile, Gfa, Pansn};

pub fn path_main(matches: &ArgMatches) {
    if check_numeric_gfafile(matches.value_of("gfa").unwrap()) {
        let graph_file = matches.value_of("gfa").unwrap();
        let output = matches.value_of("output").unwrap();
        let haplo = matches.is_present("pansn");

        let stats: Vec<&str> = matches.values_of("stats").unwrap().collect();
        let mins: Vec<&str> = matches.values_of("mins").unwrap().collect();
        let maxs: Vec<&str> = matches.values_of("maxs").unwrap().collect();
        let mins_u32 = parse_max_min(mins, false);
        let maxs_u32 = parse_max_min(maxs, true);

        let graph: Gfa<u32, (), ()> = Gfa::parse_gfa_file(graph_file);
        if graph.paths.is_empty() {
            panic!("Error: No path found in graph file")
        }
        let wrapper: Pansn<u32, (), ()> = Pansn::from_graph(&graph.paths, " ");

        let result = path_runner(&stats, &mins_u32, &maxs_u32, &graph, &wrapper, haplo);
        write_paths(&result, output);
    } else {
        panic!("Error: GFA file is not numeric");
    }
}

/// Input are the vectors, do this later
pub fn path_runner(
    stats: &Vec<&str>,
    mins_u32: &Vec<usize>,
    maxs_u32: &Vec<usize>,
    graph: &Gfa<u32, (), ()>,
    wrapper: &Pansn<u32, (), ()>,
    haplo: bool,
) -> Vec<String> {
    let f = path_stats_wrapper(graph, wrapper, haplo);

    let mut result = Vec::with_capacity(f.len());

    for x in f.iter() {
        let mut all_good = true;
        for x1 in x.1.iter() {
            if stats.contains(&&*x1.0) {
                let ff = stats.iter().position(|&y| &y == &&*x1.0).unwrap();
                if x1.1 > maxs_u32[ff] as f64 || x1.1 < mins_u32[ff] as f64 {
                    all_good = false;
                }
            }
        }
        if all_good {
            result.push(x.0.clone());
        }
    }

    result
}

pub fn parse_max_min(val: Vec<&str>, is_max: bool) -> Vec<usize> {
    let mut result_usize: Vec<usize> = Vec::with_capacity(val.len());
    if is_max {
        for x in val.iter() {
            if x != &"-" {
                result_usize.push(x.parse().unwrap());
            } else {
                result_usize.push(usize::MAX);
            }
        }
    } else {
        for x in val.iter() {
            if x != &"-" {
                result_usize.push(x.parse().unwrap());
            } else {
                result_usize.push(usize::MIN);
            }
        }
    }
    result_usize
}
