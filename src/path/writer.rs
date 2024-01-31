use std::fs::File;
use std::io::{BufWriter, Write};

pub fn write_paths(data: &Vec<String>, filename: &str) {
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);

    for x in data.iter() {
        writeln!(f, "{}", x).expect("Write path writer");
    }
}
