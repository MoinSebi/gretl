use std::io::{self, BufWriter, Write};
use std::fs::File;

pub fn write_yaml(data: Vec<String>, tab: Vec<&str>, filename:  &str){
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    for (d, x) in tab.iter().zip(data){
        write!(f, "{}: {}", x, d);
    }
}

pub fn write_yaml_path(data: Vec<Vec<String>>, tab: Vec<&str>, tab2: Vec<&str>, filename:  &str){
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    for (d, x) in tab.iter().zip(data){
        for (d2, x2) in tab2.iter().zip(x){
            write!(f, "{}: {}", x2, d2);
        }
    }
}