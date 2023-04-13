mod stats;
mod bootstrap;
mod core;
mod id2int;

use std::env::args;
use clap::{Arg, App, AppSettings};
use gfa_reader::Gfa;
use crate::bootstrap::bootstrap_main::bootstrap_main;
use crate::core::core_main::core_main;
use crate::id2int::id2int_main::id2int_main;
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
        .setting(AppSettings::SubcommandRequiredElseHelp)
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
            .takes_value(true)
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
            .arg(Arg::new("tsv")
                .short('t')
                .about("Tab seperated values format "))
            .arg(Arg::new("YAML")
                .short('y')
                .about("yaml format")))

        .subcommand(App::new("bootstrap")
            .arg(Arg::new("Pan-SN")
                .short('p')
                .long("pansn")
                .about("Seperate by first entry in Pan-SN spec")
                .takes_value(true))
            .arg(Arg::new("meta")
                .long("meta")
                .about("Make a meta file"))
            .arg(Arg::new("meta input")
                .long("meta-input")
                .about("Take a meta file input for specific stuff")
                .takes_value(true))
            .arg(Arg::new("meta line")
                .long("meta-line")
                .about("Take a specific line")
                .takes_value(true))
            .arg(Arg::new("level")
                .long("level")
                .about("Only calculate a specific level")
                .takes_value(true))
            .arg(Arg::new("number")
                .long("number")
                .about("Number of bootstraps")
                .takes_value(true)))
        .subcommand(App::new("core")
            .about("Graph similarity statistics"))

        .subcommand(App::new("id2int")
            .arg(Arg::new("dict")
                .long("dict")
                .about("Write a dictionary for Old->New values in this file. ")))
        .get_matches();

    // Read the graph
    let gfa = matches.value_of("gfa").unwrap();
    let output = matches.value_of("output").unwrap();
    let mut graph = Gfa::new();
    graph.read_file(&gfa);
    eprintln!("File name: {}", gfa);
    let filename = get_filename(&gfa);
    eprintln!("The filename is {}", filename);
    if let Some(ref matches) = matches.subcommand_matches("stats"){
        println!("Test");
        stats_main(matches, &graph);

    } else if let Some(ref matches) = matches.subcommand_matches("bootstrap") {
        println!("Test");
        bootstrap_main(&matches, &graph);
    } else if let Some(ref matches) = matches.subcommand_matches("core"){
        println!("Test");
        core_main(&matches, &graph, output);
    } else if let Some(ref matches) = matches.subcommand_matches("id2int"){
        println!("Test");
        id2int_main(&matches, &graph);
    }
    println!("Test");


}
