use clap::ArgMatches;
use gfa_reader::{Gfa, GraphWrapper};

pub fn read_graph(matches: &ArgMatches) -> Gfa {
    let gfa = matches.value_of("gfa").unwrap();
    let output = matches.value_of("output").unwrap();
    let mut graph = Gfa::new();
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

