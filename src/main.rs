mod stats;

use clap::{Arg, App, AppSettings};
use gfa_reader::Gfa;
use crate::stats::graph_stats::graph_stats_wrapper;
use crate::stats::helper::get_filename;
use crate::stats::path::path_stats_wrapper;
use crate::stats::stats_main::stats_main;
use crate::stats::writer::{write_tsv_path, write_yaml};

fn main() {


    let matches = App::new("gfastats")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.1.0")
        .author("Sebastian V")
        .about("GFa stats")

        .help_heading("Input options")
        .arg(Arg::new("gfa")
            .short('g')
            .long("gfa")
            .about("Input GFA file")
            .takes_value(true)
            .required(true))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .about("Output")
                 .required(true))
        .subcommand(App::new("stats")
            .arg(Arg::new("structure")
                .short('s')
                .long("structure")
                .about("Statistics based on structure of the graph"))
            .arg(Arg::new("path")
                .short('p')
                .long("path")
                .about("Path based structure"))

            .help_heading("Output options")
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .about("Output"))
            .arg(Arg::new("tsv")
                .short('t')
                .about("Tab seperated values format "))
            .arg(Arg::new("YAML")
                .short('y')
                .about("yaml format")))
        .get_matches();

    // Read the graph
    let gfa = matches.value_of("gfa").unwrap();
    let mut graph = Gfa::new();
    graph.read_file(&gfa);
    eprintln!("File name: {}", gfa);
    let filename = get_filename(&gfa);
    eprintln!("The filename is {}", filename);
    stats_main(matches, &graph);


}
