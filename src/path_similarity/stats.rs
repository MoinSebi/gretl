use gfa_reader::Gfa;
use crate::stats::helper::core1;

pub fn test(graph: &Gfa) -> Vec<(String, Vec<(usize, usize)>)>{
    let cores = core1(graph);

    let mut result = Vec::new();
    for path in graph.paths.iter(){
        let mut vv = vec![(0,0); graph.paths.len()+1];
        for node in path.nodes.iter(){
            let level = cores[&node.parse::<u32>().unwrap()] as usize;
            vv[level].0 += 1;
            vv[level].1 += graph.nodes.get(node).unwrap().len;
        }
        result.push((path.name.clone(), vv));

    }
    return result
}