use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

/// Read the meta file
///
/// Format of the meta file
/// [number of genomes, number of iteration, combination (HashSet)]
pub fn read_meta(filename: &str) -> Vec<(usize, usize, HashSet<usize>)> {
    let mut data: Vec<(usize, usize, HashSet<usize>)> = vec![];
    let file = File::open(filename).expect("cant read");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let lu = line.unwrap();
        let ls: Vec<&str> = lu.split("\t").collect();
        let f: HashSet<usize> = ls[2]
            .split(",")
            .map(|n| n.to_string().parse().unwrap())
            .collect();
        data.push((
            ls[0].to_string().parse().unwrap(),
            ls[1].to_string().parse().unwrap(),
            f,
        ));
    }
    return data;
}
