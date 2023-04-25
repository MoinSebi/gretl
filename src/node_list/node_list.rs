use clap::ArgMatches;
use gfa_reader::Gfa;
use crate::node_list::wrapper::wrapper;

pub fn nodelist_main(matches: &ArgMatches, graph: &Gfa, output: &str) {
    // Size, depth, similarity, degree in, degree out, degree_total, inversion amount,
    // path related?
    let data = wrapper(&graph);


}