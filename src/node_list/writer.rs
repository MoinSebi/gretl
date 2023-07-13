use std::collections::HashMap;
use std::io::{BufWriter, Write};
use std::fs::File;
use std::ptr::write;


pub fn make_buffer(filename: &str) -> BufWriter<File>{
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    return f
}


pub fn write_header(data: &Vec<&String>, f: &mut BufWriter<File>){
    let f1: Vec<String> = data.iter().map(|a| a.to_string()).collect();
    write!(f, "{}\t{}\n", "Nodes", f1.join("\t")).expect("hilfe");
}

pub fn write_list(data: (&str, HashMap<&String, u32>), order: &Vec<&String>, f: &mut BufWriter<File>){
    write!(f, "{}\t", data.0).expect("hilfe");

    for x in order.iter(){
        let val = data.1.get(x).unwrap();
        write!(f, "{}\t", data.1.get(x).unwrap()).expect("hilfe");
    }
    write!(f, "\n").expect("hilfe");
}