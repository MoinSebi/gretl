use clap::ArgMatches;
use gfa_reader::{NCGfa, NCPath, Pansn};

/// Main function for converting string ID to integer ID
///
/// This returns numeric, compact graph (starting node = 1)
pub fn id2int_main(matches: &ArgMatches) {
    let mut graph: NCGfa<()> = NCGfa::new();
    graph.parse_gfa_file_and_convert(matches.value_of("gfa").unwrap(), true);
    let mut _wrapper: Pansn<NCPath> = Pansn::from_graph(&graph.paths, " ");
    let output = matches.value_of("output").unwrap();
    eprintln!("ID2INT");
    eprintln!("Convert non digit node identifiers to numeric and compress ID space (not smart)");

    graph.to_file(output);
}
