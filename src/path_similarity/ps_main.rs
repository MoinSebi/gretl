use clap::ArgMatches;
use gfa_reader::{Gfa, GraphWrapper};
use crate::path_similarity::stats::accession2level;
use crate::path_similarity::writer_test::write_ps;

/// Main function for path related stats
pub fn ps_main(_matches: &ArgMatches, graph: &Gfa, graph2: &GraphWrapper, output: &str){

    let data = accession2level(graph, graph2);

    write_ps(&data, output);

}