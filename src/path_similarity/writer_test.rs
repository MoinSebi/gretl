use crate::helpers::helper::get_writer;
use log::info;

use std::io::Write;

/// Write path similarity data to a file
pub fn write_ps(data: &Vec<(String, Vec<(u32, u32)>)>, filename: &str) {
    info!("Writing path similarity data to file");
    let mut f = get_writer(filename).expect("Not able to write");
    let f1 = data[0].1.len();

    let k = "Accession\tType";
    let fk = (0..f1).map(|n| n.to_string()).collect::<Vec<String>>().join("\t");
    writeln!(f, "{}\t{}", k, fk).expect("Not able to write");

    for entry in data.iter() {
        let test1: Vec<String> = entry.1.iter().map(|n| n.0.to_string()).collect();
        writeln!(f, "{}\tN\t{}", entry.0, test1.join("\t"))
            .expect("Not able to write");
    }
    for entry in data.iter() {
        let test2: Vec<String> = entry.1.iter().map(|n| n.1.to_string()).collect();
        writeln!(f, "{}\tS\t{}", entry.0, test2.join("\t"))
            .expect("Not able to write");
    }
}
