use std::io::{BufWriter, Write};
use std::fs::File;

/// Write path similarity data to a file
pub fn write_ps(data: &Vec<(String, Vec<(u32, u32)>)>, filename: &str){
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    let f1 = data[0].1.len();
    let k = "Accession";
    let fk: Vec<String> = (0..f1).into_iter().map(|n| {let mut d = "Node:".to_string(); d.push_str(&n.to_string()); d}).collect();
    let fk1: Vec<String> = (0..f1).into_iter().map(|n| {let mut d = "Seq:".to_string(); d.push_str(&n.to_string()); d}).collect();
    write!(f, "{}\t{}\t{}\n", k, fk.join("\t"), fk1.join("\t")).expect("Not able to write");

    for entry in data.iter() {
        let test1: Vec<String> = entry.1.iter().map(|n| n.0.to_string()).collect();
        let test2: Vec<String> = entry.1.iter().map(|n| n.1.to_string()).collect();
        write!(f, "{}\t{}\t{}\n", entry.0, test1.join("\t"), test2.join("\t")).expect("Not able to write");
    }
}