use std::fs::File;
use std::io::{BufWriter, Write};

/// Write table with both information in one line
pub fn writer_core(
    data_total: Vec<(usize, usize)>,
    data_private: Vec<(String, usize, usize)>,
    filename: &str,
) {
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    // Header
    writeln!(f, "Feature\tSequence[bp]\t#Node").expect("Not able to write");

    // Write the accession based information
    for (name, nodes, seq) in data_private.iter() {
        writeln!(f, "{}\t{}\t{}", name, seq, nodes).expect("Not able to write");
    }

    // Write the overall distribution
    for (i, x) in data_total.iter().enumerate() {
        writeln!(f, "{}\t{}\t{}", i, x.1, x.0).expect("Not able to write");
    }
}
