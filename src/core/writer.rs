use crate::helpers::helper::get_writer;

use std::io::Write;

/// Write table with both information in one line
pub fn writer_core(
    data_total: Vec<(usize, usize)>,
    data_private: Vec<(String, usize, usize)>,
    filename: &str,
) {
    let mut f = get_writer(filename).expect("Not able to write");
    // Header
    writeln!(f, "Similarity\tName\tSequence[bp]\t#Node").expect("Not able to write");
    writeln!(f, "{}\t{}\t{}\t{}", 0, 0, data_total[0].1, data_total[0].0)
        .expect("Not able to write");

    for (name, nodes, seq) in data_private.iter() {
        writeln!(f, "{}\t{}\t{}\t{}", 1, name, seq, nodes).expect("Not able to write");
    }

    // Write the overall distribution
    for (i, (node, seq)) in data_total.iter().enumerate().skip(1) {
        writeln!(f, "{}\t{}\t{}\t{}", i, i, seq, node).expect("Not able to write");
    }
}
