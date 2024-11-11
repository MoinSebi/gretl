use crate::node_list::nl_wrapper::wrapper_node_list;
use clap::ArgMatches;
use gfa_reader::{check_numeric_gfafile, Gfa, Pansn};
use log::info;

/// Main function for node list (nl) subcommand
pub fn nodelist_main(matches: &ArgMatches) {
    info!("Running 'gretl node-list'");
    let mut sep = " ";
    if matches.is_present("Pan-SN") {
        sep = matches.value_of("Pan-SN").unwrap();
    }

    if check_numeric_gfafile(matches.value_of("gfa").unwrap()) {
        // Parse GFA file + Wrapper
        let mut graph: Gfa<u32, (), ()> = Gfa::parse_gfa_file(matches.value_of("gfa").unwrap());
        graph.walk_to_path(sep);
        if graph.paths.is_empty() {
            panic!("Error: No path found in graph file")
        }

        // PanSN
        let wrapper: Pansn<u32, (), ()> = Pansn::from_graph(&graph.paths, sep);

        // Other inputs
        let output = matches.value_of("output").unwrap();
        let features = vec!["Length", "Similarity", "Depth", "ND"];
        let bit_feature = get_features(matches, &features);
        info!("Graph file: {}", matches.value_of("gfa").unwrap());
        info!("Output file: {}", output);
        info!("Features: {:?}\n", features.iter().zip(bit_feature.iter()).filter(|(_, &x)| x).map(|(x, _)| *x).collect::<Vec<&str>>());

        info!("Starting node list analysis");
        // This wrapper also writes data to a file
        wrapper_node_list(&graph, &wrapper, output, &bit_feature);
        info!("Finished writing to file");
    } else {
        panic!("Error: GFA file is not numeric");
    }
}


/// Which features is needed
pub fn get_features(matches: &ArgMatches, features: &Vec<&str>) -> Vec<bool> {
    let splits = features;
    let mut split_vec = Vec::new();
    if matches.is_present("Features") {
        split_vec = matches.value_of("Features").unwrap().split(',').collect();
    }

    let mut result_vec = Vec::new();
    if split_vec.is_empty(){
        return splits.iter().map(|x| true).collect();
    }
    for x in splits.iter() {
        if split_vec.contains(x) {
            result_vec.push(true);
        } else {
            result_vec.push(false);
        }
    }
    result_vec
}