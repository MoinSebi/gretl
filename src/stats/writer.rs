use std::io::{self, BufWriter, Write};
use std::fs::File;

pub fn write_yaml(data: &Vec<String>, tab: &[&str], filename:  &str){
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    for (d, x) in data.iter().zip(tab){
        write!(f, "{}: {}\n", x, d);
    }
}

pub fn write_yaml_path(data: &Vec<(String, Vec<String>)>, tab2: &[&str], filename:  &str){
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    for x in data.iter(){
        write!(f, "Path: {}\n", x.0);
        for (d2, x2) in tab2.iter().zip(&x.1){
            write!(f, "\t-{}: {}\n", d2, x2);
        }
    }
}


pub fn write_tsv_path(data: &Vec<(String, Vec<String>)>, tab2: &[&str], filename:  &str) {
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    write!(f, "Path\t");
    for x in tab2.iter(){
        write!(f, "{}\t", *x);
    }
    write!(f, "\n");
    for x in data.iter(){
        write!(f, "{}\t", x.0);
        for y in x.1.iter(){
            write!(f, "{}\t", y);
        }
        write!(f, "\n");
    }


}