use crate::path_similarity::stats::accession2level;
use crate::path_similarity::writer_test::write_ps;
use clap::ArgMatches;
use gfa_reader::{NCGfa, NCPath, Pansn};

/// Main function for path related stats
pub fn ps_main(matches: &ArgMatches) {
    let mut graph: NCGfa<()> = NCGfa::new();
    graph.parse_gfa_file_and_convert(matches.value_of("gfa").unwrap(), true);
    let mut wrapper: Pansn<NCPath> = Pansn::from_graph(&graph.paths, " ");
    let data = accession2level(&graph, &wrapper);
    let output = matches.value_of("output").unwrap();

    write_ps(&data, output);
}
