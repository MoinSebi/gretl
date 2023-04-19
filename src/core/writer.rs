use std::io::{BufWriter, Write};
use std::fs::File;

/// Write table with both information in one line
pub fn writer_core(data: Vec<(usize, usize)>, data2: Vec<(String, usize, usize)>, filename: &str){
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    write!(f, "Featrure\tSequence[bp]\t#Node\n").expect("Not able to write");
    for  (name, nodes, seq) in data2.iter(){
        write!(f, "{}\t{}\t{}\n", name, seq, nodes).expect("Not able to write");
    }
    for  (i, x) in data.iter().enumerate(){
        write!(f, "{}\t{}\t{}\n", i, x.1, x.0).expect("Not able to write");
    }

}