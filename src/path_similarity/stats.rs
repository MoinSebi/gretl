use gfa_reader::{Gfa, GraphWrapper};
use crate::stats::helper::{core1, core2};


/// Compute the amount of sequence in each similarity level
pub fn accession2level(graph: &Gfa, graph2: &GraphWrapper) -> Vec<(String, Vec<(usize, usize)>)>{
    let cores = core2(graph2, graph);

    let mut result = Vec::new();
    for string2path in graph2.genomes.iter(){

        let mut vv = vec![(0,0); graph2.genomes.len()+1];
        for path in string2path.1.iter() {
            for node in path.nodes.iter() {
                let level = cores[&node.parse::<u32>().unwrap()] as usize;
                vv[level].0 += 1;
                vv[level].1 += graph.nodes.get(node).unwrap().len;
            }
        }
        result.push((string2path.0.clone(), vv));
    }
    return result
}