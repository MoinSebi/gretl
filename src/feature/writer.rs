use std::io::{BufWriter, Write};
use std::fs::File;

pub fn write_list(data: &Vec<usize>, filename: &str){
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);

    for x in data.iter(){
        write!(f, "{}\n", x);
    }
}