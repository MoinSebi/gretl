use crate::feature::writer::write_list;
use crate::helpers::helper::{calculate_depth, node_degree};
use crate::stats::graph_stats::calculate_node_size;
use clap::ArgMatches;
use gfa_reader::{NCGfa, NCPath, Pansn};

pub fn feature_main2(matches: &ArgMatches) {
    // input, output
    let graph_file = matches.value_of("gfa").unwrap();
    let file_output = matches.value_of("output").unwrap();

    let mut maxlen: i128 = matches.value_of("max-len").unwrap_or("-9").parse().unwrap();
    let minlen: i128 = matches.value_of("min-len").unwrap_or("-9").parse().unwrap();
    let mut maxdegree: i128 = matches
        .value_of("max-depth")
        .unwrap_or("-9")
        .parse()
        .unwrap();
    let mindegree: i128 = matches
        .value_of("min-depth")
        .unwrap_or("-9")
        .parse()
        .unwrap();
    let mut maxdepth: i128 = matches
        .value_of("max-depth")
        .unwrap_or("-9")
        .parse()
        .unwrap();
    let mindepth: i128 = matches
        .value_of("min-depth")
        .unwrap_or("-9")
        .parse()
        .unwrap();
    let _pansn = matches.value_of("pansn").unwrap_or(" ");

    let mut graph: NCGfa<()> = NCGfa::new();
    graph.parse_gfa_file_and_convert(graph_file, true);
    let wrapper: Pansn<NCPath> = Pansn::from_graph(&graph.paths, " ");
    println!("{}", minlen);

    if maxdegree == maxdepth
        && maxdegree == maxlen
        && maxdegree == -9
        && mindepth == maxdegree
        && mindegree == maxdegree
        && minlen == maxdegree
    {
        return;
    } else {
        if maxlen == -9 {
            maxlen = i128::MAX;
        }
        if maxdepth == -9 {
            maxdepth = i128::MAX;
        }
        if maxdegree == -9 {
            maxdegree = i128::MAX;
        }
    }
    println!("{}", maxlen);

    let result = runner1(
        &graph, &wrapper, mindepth, mindegree, minlen, maxlen, maxdegree, maxdepth,
    );
    println!("{:?}", result);
    println!("{:?}", file_output);

    write_list(&result, file_output);
}

pub fn runner1(
    graph: &NCGfa<()>,
    wrapper: &Pansn<NCPath>,
    mindepth: i128,
    mindegree: i128,
    minlen: i128,
    maxlen: i128,
    maxdegree: i128,
    maxdepth: i128,
) -> Vec<usize> {
    let paths = wrapper.get_path_genome();

    let mut result = Vec::new();
    let size = calculate_node_size(graph);
    let degree = node_degree(graph);
    let depth = calculate_depth(&paths, graph);
    for (i, (s, (deg, dep))) in size
        .iter()
        .zip(degree.2.iter().zip(depth.iter()))
        .enumerate()
    {
        if *s as i128 > minlen
            && *deg as i128 > mindegree
            && *dep as i128 > mindepth
            && (*s as i128) < maxlen
            && (*deg as i128) < maxdegree
            && (*dep as i128) < maxdepth
        {
            result.push(i)
        }
    }
    result
}
