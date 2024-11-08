use std::fs::File;
use std::io::{BufWriter, Write};
use crate::helpers::helper::get_writer;

pub fn write_paths(data: &Vec<String>, filename: &str) {
    let mut writer = get_writer(filename).expect("Write path writer");

    for x in data.iter() {
        writeln!(writer, "{}", x).expect("Write path writer");
    }
}
