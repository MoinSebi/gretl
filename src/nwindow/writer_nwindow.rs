use crate::helpers::helper::get_writer;
use gfa_reader::Segment;

use std::io::Write;

/// Write
pub fn write_list(data: &Vec<[u128; 4]>, filename: &str, nodes: &Vec<Segment<u32, ()>>) {
    let mut f = get_writer(filename).expect("Not able to write");
    writeln!(f, "Node_id\tNodes\tSequences\tJumps").expect("Not able to write");

    for (x, _node) in data.iter().zip(nodes.iter()) {
        writeln!(f, "{}\t{}\t{}\t{}", x[3], x[0], x[1], x[2]).expect("Not able to write");
    }
    //writeln!(f).expect("Not able to write");
}
