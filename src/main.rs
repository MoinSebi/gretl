mod block;
mod bootstrap;
mod core;
mod find;
mod helpers;
mod id2int;
mod logging;
mod node_list;
mod nwindow;
mod path_similarity;
mod sliding_window;
mod stats;

use crate::block::block_main::block_main;
use crate::bootstrap::bootstrap_main::bootstrap_main;
use crate::core::core_main::core_main;
use crate::find::find_main::find_main;
use crate::id2int::id2int_main::id2int_main;
use crate::logging::newbuilder;
use crate::node_list::node_list_main::nodelist_main;
use crate::nwindow::nwindow_main::nwindow_main;
use crate::path_similarity::ps_main::ps_main;
use crate::sliding_window::sliding_window_main::window_main;
use crate::stats::stats_main::stats_main;
use clap::{App, AppSettings, Arg};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("gretl")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.1.0")
        .author("Sebastian V")
        .about("GFA stats")
        .setting(AppSettings::SubcommandRequiredElseHelp)

        // Subcommand for normal stats for one graph
        .subcommand(App::new("stats")
            .about("Basic graph statistics for a single graph")
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
            .arg(Arg::new("PanSN")
                .long("pansn")
                .about("Separator for Pan-SN spec")
                .takes_value(true)
                .default_value("\n"))
            .arg(Arg::new("Haplo")
                .long("haplo")
                .about("Make stats for each haplotype (not sample, not path). Only in combination with Pan-SN"))
            .arg(Arg::new("bins")
                .long("bins")
                .about("Size of bins. Example: Format 10,20,30 -> (0-10, 11-20, 30+)[default: 1,50,100,1000]")
                .takes_value(true))
            .arg(Arg::new("path")
                .short('p')
                .long("path")
                .about("Report path statistics (default:off -> report graph stats)"))
            .arg(Arg::new("YAML")
                .short('y')
                .about("Report output in YAML format (default:off -> report in tsv)")))

        .subcommand(
            App::new("block")
                .version("1.0.1")
                .about("Statistics on pangenome blocks")

                .help_heading("Input parameters")
                .arg(
                    Arg::new("gfa")
                        .display_order(1 )
                        .short('g')
                        .long("graph")
                        .about("GFA input file")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("PanSN")
                        .long("pansn")
                        .about("PanSN-spec separator")
                        .takes_value(true)
                        .default_value("\n")
                )


                .help_heading("Block definition parameter")
                .arg(
                    Arg::new("node-window")
                        .display_order(2)
                        .short('w')
                        .long("node-window")
                        .about("Window size (in nodes) [defaul: 1000]")
                        .takes_value(true)
                )
                .arg(
                    Arg::new("node-step")
                        .display_order(1)
                        .short('s')
                        .long("node-step")
                        .about("Node step [default: 1000]")
                        .takes_value(true)
                )

                .arg(
                    Arg::new("sequence-window")
                        .long("sequence-window")
                        .about("Sequence window")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("sequence-step")
                        .long("sequence-step")
                        .about("Sequence step")
                        .takes_value(true)
                )



                .help_heading("Block extension criteria")
                .arg(Arg::new("distance")
                    .short('d')
                    .long("distance")
                    .about("Distance till breaking the block [bp]")
                    .takes_value(true)
                    .default_value("10000"))


                .help_heading("Output parameter")
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .about("Output file name")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("threads")
                        .long("threads")
                        .short('t')
                        .about("Number of threads")
                        .takes_value(true)
                        .default_value("1")
                )
                .arg(
                    Arg::new("blocks")
                        .long("blocks")
                        .short('b')
                        .about("Output all blocks in a this file")
                ),
        )


        .subcommand(App::new("bootstrap")
            .about("Bootstrap approach")
            .help_heading("Input options")
            .arg(Arg::new("gfa")
                .short('g')
                .long("gfa")
                .about("Input GFA file")
                .takes_value(true)
                .required(true))
            .arg(Arg::new("Pan-SN")
                .long("pansn")
                .about("Separate by first entry in Pan-SN spec")
                .takes_value(true))

            .arg(Arg::new("meta input")
                .long("meta-input")
                .about("Take a specific meta file as input")
                .takes_value(true))
            .arg(Arg::new("Threads")
                .long("threads")
                .short('t')
                .takes_value(true)
                .about("Number of threads")
                .default_value("1"))
            .arg(Arg::new("nodes")
                .long("nodes")
                .about("Run bootstrap only on these nodes")
                .takes_value(true))


            .help_heading("Modifications")
            .arg(Arg::new("meta line")
                .long("meta-line")
                .about("Take a specific line of the meta file (only works when meta file is provided)")
                .takes_value(true)
                .hidden(true))

            .arg(Arg::new("level")
                .long("level")
                .about("Calculate a specific level")
                .takes_value(true)
                .hidden(true))
            .arg(Arg::new("number")
                .long("number")
                .about("How many bootstraps do you want to run")
                .takes_value(true))

            .help_heading("Output options")
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .about("Output")
                .takes_value(true)
                .required(true))
            .arg(Arg::new("meta-output")
                .long("meta-output")
                .about("Output meta file")
                .takes_value(true)))








        // Subcommand for similarity level
        .subcommand(App::new("core")
            .about("General graph similarity statistics")
            .help_heading("Input options")
            .arg(Arg::new("gfa")
                .short('g')
                .long("gfa")
                .about("Input GFA file")
                .takes_value(true)
                .required(true))
            .arg(Arg::new("Pan-SN")
                .long("pansn")
                .about("Separate by first entry in Pan-SN spec")
                .default_value("\n")
                .takes_value(true))

            .help_heading("Counting options")
            .arg(Arg::new("statistics")
                .short('s')
                .long("stats")
                .about("similarity, depth, node degree")
                .default_value("similarity")
                .takes_value(true))

            .help_heading("Performance options")
            .arg(Arg::new("threads")
                .short('t')
                .long("threads")
                .about("Number of threads to use")
                .takes_value(true)
                .default_value("1"))

            .help_heading("Output options")
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .about("Output")
                .takes_value(true)
                .required(true))
        )

        // Subcommand for detailed similarity
        .subcommand(App::new("ps")
            .about("How much core, soft and private genome is in each sample?")
            .help_heading("Input options")
            .arg(Arg::new("gfa")
                .short('g')
                .long("gfa")
                .about("Input GFA file")
                .takes_value(true)
                .required(true))

            .arg(Arg::new("PanSN")
                .long("pansn")
                .about("Separate by first entry in Pan-SN spec")
                .takes_value(true)
                .default_value("\n"))

        .help_heading("Performance options")
        .arg(Arg::new("threads")
            .short('t')
            .long("threads")
            .about("Number of threads to use")
            .takes_value(true)
            .default_value("1"))

        .help_heading("Output options")
        .arg(Arg::new("output")
                .short('o')
                .long("output")
                .about("Output")
                .takes_value(true)
                .required(true)))




        // Subcommand to convert string graph to numeric
        .subcommand(App::new("id2int")
            .about("Convert node identifier to numeric values (not sorted)")

            .help_heading("Input options")
            .arg(Arg::new("gfa")
                .short('g')
                .long("gfa")
                .about("Input GFA file")
                .takes_value(true)
                .required(true))

            .help_heading("Output options")
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .about("Output file name")
                .takes_value(true)
                .required(true))
            .arg(Arg::new("dict")
                .long("dict")
                .about("Write a dictionary for Old->New identifiers in this file.")
                .takes_value(true)
                .short('d')))

        .subcommand(App::new("window")
            .about("Sliding window analysis (path-centric)")
            .help_heading("Input options")
            .arg(Arg::new("gfa")
                .short('g')
                .long("gfa")
                .about("Input GFA file")
                .takes_value(true)
                .required(true))
            .arg(Arg::new("Pan-SN")
                .long("pansn")
                .about("Separate by first entry in Pan-SN spec")
                .takes_value(true)
                .default_value("\n"))


            .help_heading("Window parameter")
            .arg(Arg::new("window-size")
                .short('w')
                .long("window-size")
                .about("Size of a single window [default: 100000]")
                .takes_value(true))
            .arg(Arg::new("step-size")
                .short('s')
                .long("step-size")
                .about("Step size [default: 100000]")
                .takes_value(true))

            .help_heading("Other options")
            .arg(Arg::new("node")
                .short('n')
                .long("node")
                .about("Window on node level ([default: off] -> on sequence)"))
            .arg(Arg::new("metric")
                .short('m')
                .long("metric")
                .about("Which metric do you wanna have reported? Example: 'similarity', 'nodesize', 'depth' [default: similarity]")
                .takes_value(true))
            .arg(Arg::new("threads")
                .short('t')
                .long("threads")
                .about("Number of threads")
                .takes_value(true)
                .default_value("1"))

            .help_heading("Output options")
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .about("Output")
                .takes_value(true)
                .required(true)))


        .subcommand(App::new("nwindow")
            .about("Sliding window along the nodes")
            .help_heading("Input options")
            .arg(Arg::new("gfa")
                .short('g')
                .long("gfa")
                .about("Input GFA file")
                .takes_value(true)
                .required(true))
            .arg(Arg::new("Pan-SN")
                .long("pansn")
                .about("Seperate by first entry in Pan-SN spec")
                .takes_value(true))

            .help_heading("Window criteria options")
            .arg(Arg::new("step")
                .long("step")
                .about("Number of steps away from the starting node")
                .takes_value(true))
            .arg(Arg::new("sequence")
                .long("sequence")
                .about("Amount of sequence away from the starting node")
                .takes_value(true))
            .arg(Arg::new("jump")
                .long("jumps")
                .about("Sum of 'id jumps' away from the starting node"))


            .help_heading("Window summary options")
            .arg(Arg::new("number of nodes")
                .long("node-number"))
            .arg(Arg::new("sequence length")
                .long("sequence-length"))
            .arg(Arg::new("sum-of-jumps")
                .long("jumps-summary"))

            .help_heading("Output option")
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .about("Output")
                .takes_value(true)
                .required(true))

        )




        .subcommand(App::new("node-list")
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
                .about("Separate by first entry in Pan-SN spec")
                .takes_value(true))
            .about("Some information about each node")
            .arg(Arg::new("Features")
                .short('f')
                .long("feature")
                .takes_value(true)
                .about("Name the features you need. If nothing is used, report everything. Example -f Length, Core")))



        .subcommand(App::new("find")
            .about("Find features in the graph and return a BED file for further analysis")
            .arg(Arg::new("gfa")
                .short('g')
                .long("gfa")
                .about("Input GFA file")
                .takes_value(true)
                .required(true)
            )
            .arg(Arg::new("features")
                .short('f')
                .long("features")
                .about("Input feature file (one feature per line). Example: 1 (node), 1+ (dirnode), 1+2+ (edge)")
                .takes_value(true)
                .required(true)
            )
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .about("Output file")
                .takes_value(true)
                .required(true)
            )
            .arg(Arg::new("length")
                .short('l')
                .long("length")
                .about("Length")
                .takes_value(true)
                .required(true)
            )
        )

        .get_matches();

    // Read the graph
    newbuilder(&matches);

    if let Some(matches) = matches.subcommand_matches("core") {
        core_main(matches)
    } else if let Some(matches) = matches.subcommand_matches("bootstrap") {
        bootstrap_main(matches);
        Ok(())
    } else if let Some(matches) = matches.subcommand_matches("stats") {
        stats_main(matches);
        Ok(())
    } else if let Some(matches) = matches.subcommand_matches("id2int") {
        id2int_main(matches)
    } else if let Some(matches) = matches.subcommand_matches("ps") {
        ps_main(matches);
        Ok(())
    } else if let Some(matches) = matches.subcommand_matches("node-list") {
        nodelist_main(matches);
        Ok(())
    } else if let Some(matches) = matches.subcommand_matches("window") {
        window_main(matches);
        Ok(())
    } else if let Some(matches) = matches.subcommand_matches("nwindow") {
        nwindow_main(matches);
        Ok(())
    } else if let Some(matches) = matches.subcommand_matches("find") {
        find_main(matches);
        Ok(())
    } else if let Some(matches) = matches.subcommand_matches("block") {
        block_main(matches)
    } else {
        panic!("Error: Subcommand not found")
    }
}
