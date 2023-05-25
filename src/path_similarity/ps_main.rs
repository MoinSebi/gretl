use clap::ArgMatches;
use gfa_reader::{Gfa, GraphWrapper};
use crate::helpers::graphs::{make_wrapper, read_graph};
use crate::path_similarity::stats::accession2level;
use crate::path_similarity::writer_test::write_ps;
use crate::stats::helper::get_filename;

/// Main function for path related stats
pub fn ps_main(matches: &ArgMatches){
    let graph = read_graph(matches);
    let gw = make_wrapper(&graph, matches);
    let data = accession2level(&graph, &gw);
    let output = matches.value_of("output").unwrap();


    write_ps(&data, output);

}