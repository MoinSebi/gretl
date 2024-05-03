use std::fs::File;
use std::io::{BufWriter, Write};
use gfa_reader::Segment;

pub fn make_buffer(filename: &str) -> BufWriter<File> {
    let f = File::create(filename).expect("Unable to create file");

    BufWriter::new(f)
}

/// Write
pub fn write_list(data: &Vec<[u128; 3]>, f: &mut BufWriter<File>, nodes: &Vec<Segment<u32, ()>>) {
    writeln!(f, "nodeid\tnode\tsequence\tjumps").expect("hilfe");

    for (x, node) in data.iter().zip(nodes.iter()) {
        writeln!(f, "{}\t{}\t{}\t{}", node.id, x[0], x[1], x[2]).expect("hilfe");
    }
    writeln!(f).expect("hilfe");
}
