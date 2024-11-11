use std::error::Error;
use gfa_reader::Segment;
use std::fs::File;
use std::io::{BufWriter, Write};
use crate::helpers::helper::get_writer;

pub fn write_wrapper(data: &[[u32; 4]], feature: &Vec<bool>, mask: &Vec<bool>, header: Vec<&str>, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut writer_test = get_writer(filename)?;
    let f1: Vec<String> = feature.iter().zip(header.iter()).filter(|(x, _)| **x).map(|(_, x)| x.to_string()).collect();
    let f2: String = f1.join("\t");

    writeln!(writer_test, "{}", f2);
    for (i, x) in data.iter().enumerate() {
        if mask[i] {
            let f1: Vec<String> = feature.iter().zip(x.iter()).filter(|(x, _)| **x).map(|(_, x)| x.to_string()).collect();
            let f2: String = f1.join("\t");
            writeln!(writer_test, "{}", f2);
        }
    }
    Ok(())
}

