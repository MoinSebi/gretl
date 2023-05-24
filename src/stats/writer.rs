use std::io::{BufWriter, Write};
use std::fs::File;

/// Write function for graph stats in yaml
pub fn write_yaml(data: &Vec<String>, tab: &[&str], filename:  &str){
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    for (d, x) in data.iter().zip(tab){
        write!(f, "{}: {}\n", x, d).expect("Not able to write");
    }
}

/// Write function for graph stats in tsv
pub fn write_tsv(data: &Vec<String>, tab: &[&str], filename: &str){
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    for y in tab.iter(){
        write!(f, "{}\t", y).expect("Not able to write");
    }
    write!(f, "\n").expect("Not able to write");

    for y in data.iter(){
        write!(f, "{}\t", y).expect("Not able to write");
    }
    write!(f, "\n").expect("Not able to write");

}




/// Write function for path stats in yaml
pub fn write_yaml_path(data: &Vec<(String, Vec<String>)>, tab2: &[&str], filename:  &str){
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    for x in data.iter(){
        write!(f, "Path: {}\n", x.0).expect("Not able to write");
        for (d2, x2) in tab2.iter().zip(&x.1){
            write!(f, "\t- {}: {}\n", d2, x2).expect("Not able to write");
        }
    }
}

/// Write function for path stats in tsv
pub fn write_tsv_path(data: &Vec<(String, Vec<String>)>, tab2: &[&str], filename:  &str) {
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    write!(f, "Path\t").expect("Not able to write");
    for x in tab2.iter(){
        write!(f, "{}\t", *x).expect("Not able to write");
    }
    write!(f, "\n").expect("Not able to write");
    for x in data.iter(){
        write!(f, "{}\t", x.0).expect("Not able to write");
        for y in x.1.iter(){
            write!(f, "{}\t", y).expect("Not able to write");
        }
        write!(f, "\n").expect("Not able to write");
    }
}