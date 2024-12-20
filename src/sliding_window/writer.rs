use crate::helpers::helper::get_writer;

use std::io::Write;

/// Write output of window command
///
/// Table:
/// - Each row is sample
/// - Each column is a bin
///
/// Comment: All rows have the same length. If vector is smaller then max_size (longest vector in the data set), add NaN
///
pub fn write_window(
    data: Vec<(String, Vec<f64>)>,
    filename: &str,
    node: bool,
    window_size: u32,
    step_size: u32,
) {
    let mut f = get_writer(filename).expect("Not able to write");
    let maxsize: usize = data.iter().map(|n| n.1.len()).max().unwrap();
    writeln!(
        f,
        "#Windows on {}, window-size {}, window-step {}",
        if node { "node" } else { "sequence" },
        window_size,
        step_size
    )
    .expect("Not able to write");
    for dat in data.iter() {
        let mut vec_s: Vec<String> = dat.1.iter().map(|n| n.to_string()).collect();
        filler(&mut vec_s, maxsize);
        writeln!(f, "{}\t{}", dat.0, vec_s.join("\t")).expect("Not able to write");
    }
}

/// Fills string vector with new NaN (this can be modular in the future too)
pub fn filler(data: &mut Vec<String>, maxsize: usize) {
    while data.len() < maxsize {
        data.push("NaN".to_string());
    }
}
