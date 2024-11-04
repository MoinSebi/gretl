use gfa_reader::Segment;
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn make_buffer(filename: &str) -> BufWriter<File> {
    let f = File::create(filename).expect("Unable to create file");

    BufWriter::new(f)
}

/// Write the header of 'gretl ps' output
pub fn write_header(data: &Vec<Segment<u32, ()>>, f: &mut BufWriter<File>) {
    let f1: Vec<String> = data
        .iter()
        .filter(|x| x.sequence.get_len() != 0)
        .map(|x| x.id.to_string())
        .collect();
    writeln!(f, "Nodes\t{}", f1.join("\t")).expect("Error: Node-list writer");
}

/// Write
pub fn write_list(data: (&str, &Vec<u32>), f: &mut BufWriter<File>, ko: &Vec<bool>) {
    write!(f, "{}\t", data.0).expect("Error: Node-list writer");

    for (x, i) in data.1.iter().zip(ko.iter()) {
        if *i {
            write!(f, "{}\t", x).expect("Error: Node-list writer");
        }
    }
    writeln!(f).expect("Error: Node-list writer");
}
