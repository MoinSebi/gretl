use crate::bootstrap::helper::random_numbers;
use gfa_reader::{Gfa, Pansn, Path};


use rayon::prelude::*;
use std::collections::HashSet;

/// Wrapper for combinations
///
/// Creates meta data for the bootstrap
/// One entry consists of: [number of genomes, number of iteration, combination (HashSet)]
pub fn combinations_maker_wrapper(
    size: &usize,
    amount: &usize,
) -> Vec<(usize, usize, HashSet<usize>)> {
    let mut data = vec![];
    for number in 2..size + 1 {
        let test_comb = combinations_maker(size, &number, amount);
        for (run, combination) in test_comb.iter().enumerate() {
            data.push((number, run, combination.clone()))
        }
    }
    data
}

/// Makes multiple random combinations
/// size = total sample size [0,1,2,3,4,5,6,7,8,9]
/// number = size of the sample (3 -> [1,2,3])
/// amount = amount og samples that should be drawn (2 -> [[1,2,3], [4,5,6]])
///
/// TODO
/// Make a check when not more different samples can be drawn (very unlikely)
pub fn combinations_maker(
    size: &usize,
    core_number: &usize,
    amount: &usize,
) -> Vec<HashSet<usize>> {
    let mut result = Vec::new();
    let mut counter = 0;
    while result.len() != *amount {
        let v = random_numbers(size, core_number);
        let v: HashSet<usize> = v.iter().cloned().collect();
        if !result.contains(&v) {
            result.push(v);
        }
        if counter > 10000 {
            return result;
        }
        counter += 1;
    }
    result
}

/// Removes lines (combinations) based on given condition (meta file or core)
pub fn reduce_meta(meta: &mut Vec<(usize, usize, HashSet<usize>)>, line: i32, core: i32) {
    if line != -1 {
        let value_to_retain = meta[line as usize].clone();
        meta.retain(|x| *x == value_to_retain);
    } else if core != -1 {
        meta.retain(|x| x.0 == core as usize);
    }
}

/// Calculation for one iteration
///
/// Take core and then remove stuff from it
pub fn one_iteration(
    graph_wrapper: &Pansn<u32, (), ()>,
    graph: &Gfa<u32, (), ()>,
    combination: &[usize],
    _information: &Vec<u32>,
    nodes: &HashSet<u32>,
    max_val: usize,
    max_sim: usize,
    index: &Vec<Vec<u32>>,
) -> (Vec<usize>, Vec<usize>) {
    // Get the paths
    let _paths = graph_wrapper.get_path_genome();

    let remove_vec = remove_values_vec(combination, index, max_val);

    let mut result: Vec<usize> = vec![0; max_sim + 1]; // Nodes
    let mut result2 = vec![0; max_sim + 1]; // Sequence

    // Add amount and sequence
    if nodes.len() == graph.segments.len() {
        for (i, x) in remove_vec.iter().enumerate() {
            if *x != 0 {
                result[*x as usize] += 1;
                result2[*x as usize] += graph.get_sequence_by_id(&(i as u32)).len();
            }
        }
    } else {
        for (i, x) in remove_vec.iter().enumerate() {
            if nodes.contains(&(i as u32 + 1)) && *x != 0 {
                result[*x as usize] += 1;
                result2[*x as usize] += graph.get_sequence_by_id(&(i as u32)).len();
            }
        }
    }
    result2.remove(0);
    result.remove(0);
    (result, result2)
}

/// Reduce vector wrapper
/// - Iterate over the genome with one combination
/// - Check if the genome is in the combination
/// - If not, remove it from the vector
pub fn remove_values_vec(comb: &[usize], index: &Vec<Vec<u32>>, len: usize) -> Vec<u32> {
    let mut info_removed = vec![0; len];
    for x in comb.iter() {
        for x1 in index[*x].iter() {
            info_removed[*x1 as usize] += 1;
        }
    }
    info_removed
}

pub fn index1(wrapper: &Vec<(String, Vec<&Path<u32, (), ()>>)>, threads: usize) -> Vec<Vec<u32>> {
    let chunk_size = (wrapper.len() + threads) / threads;

    let info_removed2: Vec<Vec<_>> = wrapper
        .par_chunks(chunk_size)
        .flat_map(|chunk| {
            chunk
                .iter()
                .map(|(_i, x312)| {
                    let mut ll: HashSet<u32> = HashSet::new();
                    for x in x312.iter() {
                        ll.extend(x.nodes.iter());
                    }
                    let mut a: Vec<_> = ll.iter().cloned().collect();
                    a.sort();
                    a
                })
                .collect::<Vec<_>>()
        })
        .collect();

    info_removed2
}
pub fn index2(wrapper: &Vec<(String, Vec<&Path<u32, (), ()>>)>) -> Vec<Vec<u32>> {
    let mut info_removed2 = Vec::new();

    let threads = 1;
    let _chunk_size = (wrapper.len() + threads) / threads;

    for (_i, x312) in wrapper.iter().enumerate() {
        let mut ll: HashSet<u32> = HashSet::new();
        for x in x312.1.iter() {
            ll.extend(x.nodes.iter());
        }
        let mut a: Vec<_> = ll.iter().cloned().collect();
        a.sort();
        info_removed2.push(a)
    }

    info_removed2
}
