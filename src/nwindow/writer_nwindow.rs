use std::fs::File;
use std::io::{BufWriter, Write};

pub fn make_buffer(filename: &str) -> BufWriter<File> {
    let f = File::create(filename).expect("Unable to create file");

    BufWriter::new(f)
}

/// Write
pub fn write_list(data: &Vec<[u128; 3]>, f: &mut BufWriter<File>) {
    writeln!(f, "node\tsequence\tjumps").expect("hilfe");

    for x in data.iter() {
        writeln!(f, "{}\t{}\t{}", x[0], x[1], x[2]).expect("hilfe");
    }
    writeln!(f).expect("hilfe");
}
