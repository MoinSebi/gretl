use crate::helpers::helper::get_writer;
use gfa_reader::Segment;

use std::io::{Write};

/// Write
pub fn write_list(data: &Vec<[u128; 3]>, filename: &str, nodes: &Vec<Segment<u32, ()>>) {
    let mut f = get_writer(filename).expect("hilfe");
    writeln!(f, "nodeid\tnode\tsequence\tjumps").expect("hilfe");

    for (x, node) in data.iter().zip(nodes.iter()) {
        writeln!(f, "{}\t{}\t{}\t{}", node.id, x[0], x[1], x[2]).expect("hilfe");
    }
    writeln!(f).expect("hilfe");
}
