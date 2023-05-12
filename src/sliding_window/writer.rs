use std::io::{BufWriter, Write};
use std::fs::File;


pub fn write_window(data: Vec<(String, Vec<u32>)>, filename: &str){


        let f = File::create(filename).expect("Unable to create file");
        let mut f = BufWriter::new(f);
        let f1 = data[0].1.len();
        let k = "header";


        for dat in data.iter() {
            let vec_s: Vec<String> = dat.1.iter().map(|n| n.to_string()).collect();
            write!(f, "{}\t{}\n", dat.0, vec_s.join("\t")).expect("Not able to write");
        }
    }
