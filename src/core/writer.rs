use std::io::{BufWriter, Write};
use std::fs::File;

/// Write table with both information in one line
pub fn writer_core(data_total: Vec<(usize, usize)>, data_private: Vec<(String, usize, usize)>, filename: &str){
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    // Header
    write!(f, "Feature\tSequence[bp]\t#Node\n").expect("Not able to write");

    // Write the accession based information
    for  (name, nodes, seq) in data_private.iter(){
        write!(f, "{}\t{}\t{}\n", name, seq, nodes).expect("Not able to write");
    }

    // Write the overall distribution
    for  (i, x) in data_total.iter().enumerate(){
        write!(f, "{}\t{}\t{}\n", i, x.1, x.0).expect("Not able to write");
    }

}