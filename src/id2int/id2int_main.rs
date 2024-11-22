
use chrono::Local;
use clap::ArgMatches;
use gfa_reader::index_file;
use log::info;
use rand::prelude::*;
use rand::Rng;
use rayon::iter::ParallelIterator;
use rayon::slice::*;
use std::collections::HashMap;

use std::fs::File;

use std::io::{BufRead, BufReader};
use std::io::{BufWriter, Seek, SeekFrom, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

/// Main function for converting string ID to integer ID
///
/// This returns numeric, compact graph (starting node = 1)
pub fn id2int_main(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    info!("Running 'gretl id2int'");
    let gfafile = matches.value_of("gfa").unwrap();
    let output = matches.value_of("output").unwrap();
    let dict = matches.is_present("dict");
    let threads = matches
        .value_of("threads")
        .unwrap()
        .parse()
        .expect("Error: Threads must be a number");

    info!("GFA file: {}", gfafile);
    info!("Report dictionary: {}", dict);
    if dict {
        info!("Dictionary file: {}", matches.value_of("dict").unwrap());
    }
    info!(
        "Output file: {}",
        if output == "-" { "stdout" } else { output }
    );

    info!("Read nodes from file and extract index");
    let (s, index, _count) = node_reader(matches.value_of("gfa").unwrap());
    info!("Create internal index");
    let index2 = create_strvec(index, &s);

    let version = get_version(matches.value_of("gfa").unwrap());
    info!("GFA format version is {}", version);
    info!("Create a converting hashmap (old id -> new id)");
    let hm = create_hashmap(&index2);
    let output = matches.value_of("output").unwrap();
    info!("Convert string ID to integer ID and write to a new file");
    read_write(matches.value_of("gfa").unwrap(), output, &hm, threads)?;

    if matches.is_present("dict") {
        info!("Write the hashmap to a file (tab separated)");
        write_hm(&hm, matches.value_of("dict").unwrap());
    }
    info!("Done");
    Ok(())
}

/// Get all nodes in the file
///
/// Return:
///    - String: all nodes concatenated in a string
///    - Vec<usize>: index of each node
///
/// Comment: String + index for better memory size
pub fn node_reader(filename: &str) -> (String, Vec<usize>, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut s = String::new();
    let mut index = Vec::new();
    let mut c = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with('S') {
            let fields: Vec<_> = line.split_whitespace().collect();
            s += fields[1];
            index.push(fields[1].len());
        }
        c += line.len() + 1;
    }
    (s, index, c)
}

/// Get the index of each node
///
/// Return:
///     - Vec<&str>: nodes
pub fn create_strvec(index: Vec<usize>, ss: &String) -> Vec<&str> {
    let mut res = Vec::new();
    let mut pos = 0;
    for i in index.iter() {
        res.push(&ss[pos..pos + i]);
        pos += i;
    }
    res
}

/// Make a hashmap from a vector of strings (&str)
/// The hashmap is used to convert string ID to integer ID
///
/// Comment: We start at 1
///
/// Return:
///    - HashMap<&str, usize>: HashMap with key = string ID and value = integer ID
pub fn create_hashmap<'a>(index: &'a Vec<&str>) -> HashMap<&'a str, usize> {
    let mut hm = std::collections::HashMap::new();
    for (i, x) in index.iter().enumerate() {
        hm.insert(*x, i + 1);
    }
    hm
}

/// Get the integer ID from a hashmap
/// This function is used to get the integer ID from a string ID (but return as String)
///
/// Return:
///    - String: integer ID
pub fn get_string_from_hm(hm: &HashMap<&str, usize>, s: &String) -> String {
    let a: &str = s;
    hm.get(a).unwrap().to_string()
}

#[derive(Debug, PartialOrd, PartialEq)]
enum DelEnum {
    Space,
    Comma,
    Walk,
}

/// Convert a string to a string with integer ID
/// Split the string at every digit, convert the digit to integer using hashmap
/// and join the string again
///
/// Return:
///    - String: string with converted ID (string -> integer)
pub fn convert_string(
    delim: &str,
    hashmap_old_new: &HashMap<&str, usize>,
    del_enum: DelEnum,
) -> Result<String, Box<dyn std::error::Error>> {
    if del_enum == DelEnum::Space {
        let string_vec = delim.split(' ').collect::<Vec<&str>>();
        Ok(hashmap_old_new
            .get(string_vec[0])
            .ok_or("This ID is not in the hashmap")
            .unwrap()
            .to_string())
    } else if del_enum == DelEnum::Comma {
        let mut dirs = Vec::new();
        let mut values = Vec::new();
        for x in delim.split(',') {
            dirs.push(x.chars().last().ok_or("Error converting ID").unwrap());
            values.push(
                hashmap_old_new
                    .get(&x.to_string()[0..x.len() - 1])
                    .ok_or("Error converting ID")
                    .unwrap()
                    .to_string(),
            );
        }
        Ok(values
            .iter()
            .zip(dirs.iter())
            .map(|(x, y)| format!("{}{}", y, x))
            .collect::<Vec<String>>()
            .join(""))
    } else {
        let mut dirs = Vec::new();
        let mut values = Vec::new();
        let mut oo = String::new();
        for x in delim.chars().skip(1) {
            if x.to_string() == ">" || x.to_string() == "<" {
                dirs.push(x);
                values.push(
                    hashmap_old_new
                        .get(oo.as_str())
                        .ok_or("Error converting ID")
                        .unwrap()
                        .to_string(),
                );
                oo = String::new();
            } else {
                oo += &x.to_string();
            }
        }
        Ok(values
            .iter()
            .zip(dirs.iter())
            .map(|(x, y)| format!("{}{}", y, x))
            .collect::<Vec<String>>()
            .join(""))
    }
}

/// Read a file and write to another file at the same time
/// - Read a line
/// - Convert all ID to integer ID
/// - Write the line to another file
///
/// Return:
///    - ()
///    - Write a new file with integer ID (same structure as the original file, but different ID)
///
/// Comment: Which entries are converted is based on the first character of the line
pub fn read_write(
    f1: &str,
    f2: &str,
    hm: &HashMap<&str, usize>,
    threads: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let index = index_file(f1);
    let last = index[index.len() - 1];
    let mut byte_index = pair_with_next(&index);
    byte_index.shuffle(&mut rand::thread_rng());
    let chunk_size = (byte_index.len() + threads - 1) / threads;

    let file_new = File::create(f2).unwrap_or_else(|_| panic!("Error opening file: {}", f1));
    let writer = BufWriter::new(file_new);
    let cc = AtomicUsize::new(0);
    let aa = Arc::new(Mutex::new(writer));

    let first_line = first_line(f1);
    let is_header = first_line.starts_with("H");
    if is_header {
        let mut a1 = aa.lock().unwrap();
        write!(a1, "{}", first_line).expect("Error writing to file");
        drop(a1);
    }


    info!("Start converting ID");
    byte_index.par_chunks(chunk_size).for_each(|x| {
        let mut string_vec: Vec<String> = Vec::new();
        for a in x.iter() {
            let file = File::open(f1).unwrap();
            let mut reader = BufReader::new(file);
            reader.seek(SeekFrom::Start(a.0 as u64)).unwrap();
            let mut pos = a.0;
            for line in reader.lines() {
                let l = line.unwrap();
                pos = pos + l.len() + 1;
                if pos > a.1 {
                    break;
                }

                let mut fields: Vec<&str> = l.split_whitespace().collect();
                match fields[0] {
                    "S" => {
                        let a = convert_string(fields[1], hm, DelEnum::Space)
                            .expect("Error converting ID");
                        fields[1] = &a;
                        string_vec.push(format!("{}\n", fields.join("\t")))
                    }
                    "L" => {
                        let a = convert_string(fields[1], hm, DelEnum::Space)
                            .expect("Error converting ID");
                        fields[1] = &a;
                        let b = convert_string(fields[3], hm, DelEnum::Space)
                            .expect("Error converting ID");
                        fields[3] = &b;
                        string_vec.push(format!("{}\n", fields.join("\t")))
                    }
                    "P" => {
                        let a = convert_string(fields[2], hm, DelEnum::Comma)
                            .expect("Error converting ID");
                        fields[2] = &a;
                        string_vec.push(format!("{}\n", fields.join("\t")))
                    }
                    "J" => {
                        let a = convert_string(fields[1], hm, DelEnum::Space)
                            .expect("Error converting ID");
                        fields[1] = &a;
                        let b = convert_string(fields[3], hm, DelEnum::Space)
                            .expect("Error converting ID");
                        fields[3] = &b;
                        string_vec.push(format!("{}\n", fields.join("\t")))
                    }
                    "W" => {
                        let a = convert_string(fields[6], hm, DelEnum::Walk)
                            .expect("Error converting ID");
                        fields[6] = &a;
                        string_vec.push(format!("{}\n", fields.join("\t")))
                    }
                    "C" => {
                        let a = convert_string(fields[1], hm, DelEnum::Space)
                            .expect("Error converting ID");
                        fields[1] = &a;
                        let b = convert_string(fields[3], hm, DelEnum::Space)
                            .expect("Error converting ID");
                        fields[3] = &b;
                        string_vec.push(format!("{}\n", fields.join("\t")))
                    }
                    "F" => {
                        let a = convert_string(fields[1], hm, DelEnum::Space)
                            .expect("Error converting ID");
                        fields[1] = &a;
                        string_vec.push(format!("{}\n", fields.join("\t")))
                    }
                    "E" => {
                        let a = convert_string(fields[2], hm, DelEnum::Space)
                            .expect("Error converting ID");
                        fields[2] = &a;
                        let n = convert_string(fields[3], hm, DelEnum::Space)
                            .expect("Error converting ID");
                        fields[3] = &n;
                        string_vec.push(format!("{}\n", fields.join("\t")))
                    }
                    "G" => {
                        let a = convert_string(fields[2], hm, DelEnum::Space)
                            .expect("Error converting ID");
                        fields[2] = &a;
                        let n = convert_string(fields[3], hm, DelEnum::Space)
                            .expect("Error converting ID");
                        fields[3] = &n;
                        string_vec.push(format!("{}\n", fields.join("\t")))
                    }
                    "U" => {
                        let mut b: Vec<String> = vec![fields[0].to_string(), fields[1].to_string()];
                        for x in fields.iter().skip(2) {
                            let a =
                                convert_string(x, hm, DelEnum::Space).expect("Error converting ID");
                            b.push(a);
                        }
                        string_vec.push(format!("{}\n", b.join("\t")))
                    }
                    "O" => {
                        let mut b: Vec<String> = vec![fields[0].to_string(), fields[1].to_string()];
                        for x in fields.iter().skip(2) {
                            let a =
                                convert_string(x, hm, DelEnum::Space).expect("Error converting ID");
                            b.push(a);
                        }
                        string_vec.push(format!("{}\n", b.join("\t")))
                    }
                    "H" => {
                        let ll = l.trim_end();
                        if !is_header {
                            string_vec.push(format!("{}\n", ll))
                        }
                    },

                    _ => panic!("This line is not recognized: {}", l),
                }
            }

            cc.fetch_add(a.1 - a.0, std::sync::atomic::Ordering::Relaxed);
            std::io::stderr().flush().unwrap();
            eprint!(
                "\r{}          Progress {:.2}%",
                Local::now().format("%d/%m/%Y %H:%M:%S %p"),
                ((cc.load(Ordering::Relaxed) as f64 / last as f64) * 100.0)
            );
        }
        let mut writer = aa.lock().unwrap();

        for x in string_vec.iter() {
            writer
                .write_all(x.as_bytes())
                .expect("Error writing to file");
        }
    });
    eprintln!();
    Ok(())
}

/// Get the version of a GFA file
/// Not sure if the header line must be the first line in the file
/// Return:
///     - f32: version number
pub fn get_version(filename: &str) -> f32 {
    let file = File::open(filename).unwrap_or_else(|_| panic!("Error opening file: {}", filename));
    let reader = BufReader::new(file);
    let mut version = 0.0;
    for line in reader.lines() {
        let line = line.unwrap();
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields[0] == "H" {
            version = fields[1].split(':').collect::<Vec<&str>>()[2]
                .parse::<f32>()
                .expect("Error parsing version number")
        }
    }
    version
}

/// Write a hashmap to a file
///
/// Return:
///     - Tab separated file with key and value
pub fn write_hm(hashmap_old_new: &HashMap<&str, usize>, filename: &str) {
    let file = File::create(filename).unwrap();
    let mut writer = std::io::BufWriter::new(file);
    for (old_id, new_id) in hashmap_old_new.iter() {
        writeln!(writer, "{}\t{}", old_id, new_id).expect("Error writing to file");
    }
}

fn pair_with_next<T: Copy>(vec: &[T]) -> Vec<(T, T)> {
    vec.iter()
        .zip(vec.iter().skip(1))
        .map(|(&x, &y)| (x, y))
        .collect()
}

fn first_line(filename: &str) -> String {
    let file = File::open(filename).expect("Error opening file");
    let mut buf_reader = BufReader::new(file);

    let mut first_line = String::new();
    buf_reader.read_line(&mut first_line).expect("Error reading file");
    first_line
}
