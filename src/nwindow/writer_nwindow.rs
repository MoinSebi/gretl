use std::fs::File;
use std::io::{BufWriter, Write};

pub fn make_buffer(filename: &str) -> BufWriter<File> {
    let f = File::create(filename).expect("Unable to create file");

    BufWriter::new(f)
}

pub fn write_header(data: &Vec<String>, f: &mut BufWriter<File>) {
    let f1: Vec<String> = data.iter().map(|a| a.to_string()).collect();
    writeln!(f, "Nodes\t{}", f1.join("\t")).expect("hilfe");
}

/// Write
pub fn write_list(data: &Vec<[u128; 3]>, f: &mut BufWriter<File>) {
    write!(f, "node\tsequence\tjumps\n").expect("hilfe");

    for x in data.iter() {
        write!(f, "{}\t{}\t{}\n", x[0], x[1], x[2]).expect("hilfe");
    }
    writeln!(f).expect("hilfe");
}
