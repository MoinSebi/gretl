use std::collections::HashMap;
use std::io::{BufWriter, Write};
use std::fs::File;
use std::ptr::write;


pub fn make_buffer(filename: &str) -> BufWriter<File>{
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    return f
}


pub fn write_header(data: &HashMap<&String, usize>, f: &mut BufWriter<File>){
    let mut a: Vec<(&&String, &usize)> = data.iter().map(|n| n).collect();
    a.sort_by_key(|n| n.1);
    write!(f, "{}\t{}\n", "Nodes", a.iter().map(|n| n.0.to_string()).collect::<Vec<String>>().join("\t")).expect("hilfe");
}

pub fn write_list(data: (&str, Vec<u32>), f: &mut BufWriter<File>){
    write!(f, "{}\t{}\n", data.0, data.1.iter().map(|n| n.to_string()).collect::<Vec<String>>().join("\t")).expect("hilfe");
}