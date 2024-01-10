use std::io::{BufWriter, Write};
use std::fs::File;

pub fn write_paths(data: &Vec<String>, filename: &str){
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);

    for x in data.iter(){
        write!(f, "{}\n", x).expect("Write path writer");
    }
}