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
pub fn write_list(data: (&str, &Vec<u32>), f: &mut BufWriter<File>) {
    write!(f, "{}\t", data.0).expect("hilfe");

    for x in data.1.iter() {
        write!(f, "{}\t", x).expect("hilfe");
    }
    writeln!(f).expect("hilfe");
}
