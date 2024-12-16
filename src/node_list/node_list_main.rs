use crate::node_list::nl_wrapper::wrapper_node_list;
use clap::ArgMatches;
use gfa_reader::{check_numeric_gfafile, Gfa, Pansn};
use log::info;

/// Main function for node list (nl) subcommand
pub fn nodelist_main(matches: &ArgMatches) {
    info!("Running 'gretl node-list'");
    let pansn = matches.value_of("Pan-SN").unwrap();
    // Other inputs
    let output = matches.value_of("output").unwrap();
    let features = vec!["Length", "Similarity", "Depth", "ND"];
    let mut bit_feature = get_features(matches, &features);
    let threads = matches
        .value_of("threads")
        .unwrap()
        .parse()
        .expect("Error: Threads must be a number");

    info!("GFA file: {}", matches.value_of("gfa").unwrap());
    info!(
        "Pan-SN: {}",
        if pansn == "\n" {
            "None".to_string()
        } else {
            format!("{:?}", pansn)
        }
    );
    info!(
        "Features: {:?}\n",
        features
            .iter()
            .zip(bit_feature.iter())
            .filter(|(_, &x)| x)
            .map(|(x, _)| *x)
            .collect::<Vec<&str>>()
    );
    info!(
        "Output file: {}",
        if output == "-" { "stdout" } else { output }
    );
    info!("Threads: {}", threads);

    info!("Numeric check");
    if check_numeric_gfafile(matches.value_of("gfa").unwrap()) {
        // Parse GFA file + Wrapper

        info!("Read GFA file");
        let mut graph: Gfa<u32, (), ()> =
            Gfa::parse_gfa_file_multi(matches.value_of("gfa").unwrap(), threads);
        graph.walk_to_path(pansn);

        // PanSN
        let wrapper: Pansn<u32, (), ()> = Pansn::from_graph(&graph.paths, pansn);

        info!("Starting node list analysis");
        // This wrapper also writes data to a file
        wrapper_node_list(&graph, &wrapper, output, &mut bit_feature);
        info!("Done");
    } else {
        panic!("The node IDs in the GFA file are not numeric");
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
    if split_vec.is_empty() {
        return splits.iter().map(|_x| true).collect();
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
