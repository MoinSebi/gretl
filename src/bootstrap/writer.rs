use std::collections::HashSet;
use std::io::{BufWriter, Write};
use std::fs::File;

pub fn write_meta(data: Vec<(usize, usize, HashSet<usize>)>, filename:  &str) {
    eprintln!("Writing meta");

    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);

    for x in data.iter(){
        write!(f, "{}\t{}\t{}\n", x.0, x.1, x.2.iter().collect::<Vec<&usize>>().iter().map(|n|n.to_string()).collect::<Vec<String>>().join(",")).expect("Not able to write");
    }


}