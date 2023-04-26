mod stats;
mod bootstrap;
mod core;
mod id2int;
mod path_similarity;
mod node_list;

use clap::{Arg, App, AppSettings};
use gfa_reader::{Gfa, GraphWrapper};
use crate::bootstrap::bootstrap_main::bootstrap_main;
use crate::core::core_main::core_main;
use crate::id2int::id2int_main::id2int_main;
use crate::node_list::node_list::nodelist_main;
use crate::path_similarity::ps_main::ps_main;
use crate::stats::helper::get_filename;
use crate::stats::stats_main::stats_main;

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
        .arg(Arg::new("Pan-SN")
          .short('s')
          .long("pansn")
          .about("Seperate by first entry in Pan-SN spec")
            .takes_value(true))


        .subcommand(App::new("stats")
            .about("Create statists ")
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
            .about("Bootstrap approach")
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
        .subcommand(App::new("ps")
            .about("Path similarity stats"))

        .subcommand(App::new("id2int")

            .about("Convert node identifier to numeric values (not sorted)")
            .arg(Arg::new("dict")
                .long("dict")
                .about("Write a dictionary for Old->New identifiers in this file.")
                .takes_value(true)
                .short('d')))
        .subcommand(App::new("node-list")

            .about("Some information about each node")
            .arg(Arg::new("Features")
                .short('f')
                .long("feature")
                .takes_value(true)
                .about("Name the features you need. If nothing is used, report everything. Example -f Length, Core")))
        .get_matches();

    // Read the graph
    let gfa = matches.value_of("gfa").unwrap();
    let output = matches.value_of("output").unwrap();
    let mut graph = Gfa::new();
    graph.read_file(&gfa);
    let mut sep = " ";
    if matches.is_present("Pan-SN"){
        sep = matches.value_of("Pan-SN").unwrap();
    }
    let mut f: GraphWrapper = GraphWrapper::new();
    f.from_ngfa(&graph, sep);
    eprintln!("File name: {}", gfa);
    let filename = get_filename(&gfa);
    eprintln!("The filename is {}", filename);
    if let Some(ref matches) = matches.subcommand_matches("stats"){
        stats_main(matches, &graph, output);

    } else if let Some(ref matches) = matches.subcommand_matches("bootstrap") {
        bootstrap_main(&matches, &graph);
    } else if let Some(ref matches) = matches.subcommand_matches("core"){
        core_main(&matches, &f, &graph, output);
    } else if let Some(ref matches) = matches.subcommand_matches("id2int"){
        id2int_main(&matches, &graph, output);
    } else if let Some(ref matches) = matches.subcommand_matches("ps"){
        ps_main(&matches, &graph, &f, output);
    } else if let Some(ref matches) = matches.subcommand_matches("node-list"){
        nodelist_main(&matches, &graph, output);
    }


}
