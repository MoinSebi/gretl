use clap::ArgMatches;
use gfa_reader::{Gfa, GraphWrapper, NCGfa, NCPath};
use crate::path_similarity::stats::accession2level;
use crate::path_similarity::writer_test::write_ps;
use crate::stats::helper::get_filename;

/// Main function for path related stats
pub fn ps_main(matches: &ArgMatches){
    let mut graph: NCGfa<()> = NCGfa::new();
    graph.parse_gfa_file_and_convert(matches.value_of("gfa").unwrap(), true);
    let mut wrapper: GraphWrapper<NCPath> = GraphWrapper::new();
    wrapper.from_gfa(&graph.paths, " ");
    let data = accession2level(&graph, &wrapper);
    let output = matches.value_of("output").unwrap();


    write_ps(&data, output);

}