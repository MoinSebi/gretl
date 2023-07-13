use clap::ArgMatches;
use gfa_reader::{Gfa, GraphWrapper, NCGfa, read_nodes};

pub fn read_graph(matches: &ArgMatches) -> NCGfa {
    let gfa = matches.value_of("gfa").unwrap();
    let mut is_digit = read_nodes(&gfa);
    let mut graph = NCGfa::new();
    if is_digit{
        graph.parse_gfa_file(&gfa);
    } else {
        let mut string_graph: Gfa<()> = Gfa::new();
        string_graph.parse_gfa_file(&gfa);
        let a = graph.make_mapper(&mut string_graph);
        graph.convert_with_mapper(a, &gfa);
    }
    graph.read_file(&gfa);
    return graph
}

pub fn make_wrapper<'a>(graph: &'a Gfa, matches: &ArgMatches) -> GraphWrapper<'a>{
    let mut sep = " ";
    if matches.is_present("Pan-SN"){
        sep = matches.value_of("Pan-SN").unwrap();
    }
    let mut f: GraphWrapper = GraphWrapper::new();
    f.from_ngfa(&graph, sep);
    //return (graph, f)
    return f
}

