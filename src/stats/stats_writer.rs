use std::io::{BufWriter, Write};
use std::fs::File;

/// Write statistics in a YAML file
///
/// Input:
pub fn write_graph_yaml(data: &Vec<(String, String)>, filename:  &str){
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    for (d, x) in data.iter(){
        write!(f, "{}: {}\n", d, x).expect("Not able to write");
    }
}



/// Write statistics in tab-separated file (tsv)
pub fn write_graph_tsv(data: &Vec<(String, String)>, filename: &str){
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    for (column_name, value_) in data.iter().take(data.len()-1){
        write!(f, "{}\t", column_name).expect("Not able to write");
    }
    write!(f, "{}\n", data[data.len()-1].0).expect("Not able to write");

    for (column_name_, value) in data.iter().take(data.len()-1){
        write!(f, "{}\t", value).expect("Not able to write");
    }
    write!(f, "{}\n", data[data.len()-1].1).expect("Not able to write");




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