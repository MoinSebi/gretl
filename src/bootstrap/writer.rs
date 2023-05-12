use std::cmp::max;
use std::collections::HashSet;
use std::io::{BufWriter, Write};
use std::fs::File;


/// Write the meta file
pub fn write_meta(data: Vec<(usize, usize, HashSet<usize>)>, filename:  &str) {
    eprintln!("Writing meta");

    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);

    for x in data.iter(){
        write!(f, "{}\t{}\t{}\n", x.0, x.1, x.2.iter().collect::<Vec<&usize>>().iter().map(|n|n.to_string()).collect::<Vec<String>>().join(",")).expect("Not able to write");
    }
}


/// Write output
pub fn write_output(data: Vec<(usize, usize, (Vec<usize>, Vec<usize>))>, filename:  &str) {
    eprintln!("Writing meta");
    let max_len = data.iter().map(|n| n.2.1.len()).max().unwrap();


    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);

    for x in data.iter(){
        let mut y = x.2.0.iter().collect::<Vec<&usize>>().iter().map(|n|n.to_string()).collect::<Vec<String>>();
        let mut  y2 = x.2.1.iter().collect::<Vec<&usize>>().iter().map(|n|n.to_string()).collect::<Vec<String>>();
        write!(f, "{}\t{}\t{}\t{}\n", x.0, x.1, fillerback(&mut y, max_len), fillerback(&mut y2, max_len)).expect("Not able to write");
    }
}

pub fn fillerback(stri: &mut Vec<String>, maxlen: usize) -> String{
    let mut act_len = stri.len();
    let mut a = stri.join("\t");
    while act_len < maxlen{
        a = a + "\t";
        act_len += 1;
    }
    a
}

