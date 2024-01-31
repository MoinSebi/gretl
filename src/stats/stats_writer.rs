use std::fs::File;
use std::io::{BufWriter, Write};

/// Write statistics in a YAML file
///
/// Input:
pub fn write_tsv_graph(data: &Vec<(String, String)>, filename: &str) {
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    for (d, x) in data.iter() {
        writeln!(f, "{}: {}", d, x).expect("Not able to write");
    }
}

/// Write statistics in tab-separated file (tsv)
pub fn write_yaml_graph(data: &Vec<(String, String)>, filename: &str) {
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    for (column_name, _value) in data.iter().take(data.len() - 1) {
        write!(f, "{}\t", column_name).expect("Not able to write");
    }
    writeln!(f, "{}", data[data.len() - 1].0).expect("Not able to write");

    for (_column_name_, value) in data.iter().take(data.len() - 1) {
        write!(f, "{}\t", value).expect("Not able to write");
    }
    writeln!(f, "{}", data[data.len() - 1].1).expect("Not able to write");
}

/// Write function for path stats in yaml
pub fn write_yaml_path(data: &Vec<(String, Vec<(String, String)>)>, filename: &str) {
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    for x1 in data.iter() {
        writeln!(f, "{}:", x1.0).expect("Not able to write");
        for (d, x) in x1.1.iter() {
            writeln!(f, "- {}: {}", d, x).expect("Not able to write");
        }
    }
}

/// Write function for path stats in tsv
pub fn write_tsv_path(data: &Vec<(String, Vec<(String, String)>)>, filename: &str) {
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    write!(f, "Path\t").expect("Not able to write");
    for x in data.iter() {
        for y in x.1.iter().take(x.1.len() - 1) {
            write!(f, "{}\t", y.0).expect("Not able to write");
        }
        writeln!(f, "{}", x.1[x.1.len() - 1].0).expect("Not able to write");
        break;
    }
    for data1 in data.iter() {
        write!(f, "{}\t", data1.0).expect("Not able to write");
        for (_column_name, value_) in data1.1.iter().take(data1.1.len() - 1) {
            write!(f, "{}\t", value_).expect("Not able to write");
        }
        writeln!(f, "{}", data1.1[data1.1.len() - 1].1).expect("Not able to write");
    }
}
