use std::collections::HashSet;
use gfa_reader::Gfa;
use crate::bootstrap::bootstrap_main::combinations_maker;

/// Make a meta file
/// It's a table (tab-sep)
/// -core
/// -run
/// -combination(comma sep)
///
///
pub fn make_meta(graph: &Gfa, amount: usize) -> Vec<(usize, usize, HashSet<usize>)>{
    let mut f = Vec::new();

    for y in 2..graph.paths.len()+1 {
        let test_comb = combinations_maker(&graph.paths.len(), &y, &amount);
        test_comb.iter().for_each(|n| f.push((graph.paths.len(), y, n.clone())))
    }

    return f
}