use clap::ArgMatches;
use gfa_reader::Gfa;
use crate::path_similarity::stats::test;
use crate::path_similarity::writer_test::write_ps;

/// Main function for node id to integer function
pub fn ps_main(_matches: &ArgMatches, graph: &Gfa, output: &str){

    let data = test(graph);

    write_ps(&data, output);

}