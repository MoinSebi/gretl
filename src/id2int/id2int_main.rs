use clap::ArgMatches;
use gfa_reader::{NCGfa, NCPath, Pansn};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::io::Write;
use chrono::Local;
use log::info;

/// Main function for converting string ID to integer ID
///
/// This returns numeric, compact graph (starting node = 1)
pub fn id2int_main(matches: &ArgMatches) {
    info!("Starting id2int_main");
    info!("Read nodes + index");
    let (s, index, count) = node_reader(matches.value_of("gfa").unwrap());
    let index2 = create_strvec(index, &s);

    let version = get_version(matches.value_of("gfa").unwrap());
    info!("Version is {}", version);
    info!("Make hashmap");
    let hm = create_hashmap(&index2);
    let output = matches.value_of("output").unwrap();
    info!("Read and write");
    read_write(matches.value_of("gfa").unwrap(), output, &hm, &count);

    if matches.is_present("dict") {
        write_hm(&hm, matches.value_of("dict").unwrap());
    }
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
        let mut fields: Vec<_> = line.split_whitespace().collect();
        if fields[0] == "S" {
            s += fields[1];
            index.push(fields[1].len());
        }
        c += 1;
    }
    return (s, index, c);
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
    return res;
}


/// Make a hashmap from a vector of strings (&str)
/// The hashmap is used to convert string ID to integer ID
///
/// Return:
///    - HashMap<&str, usize>: HashMap with key = string ID and value = integer ID
pub fn create_hashmap<'a>(index: &'a Vec<&str>) -> HashMap<&'a str, usize> {
    let mut hm = std::collections::HashMap::new();
    for (i, x) in index.iter().enumerate() {
        hm.insert(*x, i+1);
    }
    return hm;
}

/// Get the integer ID from a hashmap
/// This function is used to get the integer ID from a string ID (but return as String)
///
/// Return:
///    - String: integer ID
pub fn get_string_from_hm(hm: &HashMap<&str, usize>, s: &String) -> String{
    let a: &str = &s;
    hm.get(a).unwrap().to_string()
}


/// Convert a string to a string with integer ID
/// Split the string at every digit, convert the digit to integer using hashmap
/// and join the string again
///
/// Return:
///    - String: string with converted ID (string -> integer)
pub fn convert_string(k: &str, hm: &HashMap<&str, usize>) -> String {
    let mut res = Vec::new();
    let mut is_digit = false;
    let char_first = k.chars().nth(0).unwrap();
    if char_first.is_digit(10) {
        is_digit = true;
    }
    let mut char1 = k.chars().nth(0).unwrap().to_string();

    for c in k.chars().skip(1) {
        if c.is_digit(10) {
            if is_digit {
                char1 += &c.to_string();
            } else {
                res.push(char1);
                char1 = c.to_string();
            }
            is_digit = true;
        } else {
            if is_digit {
                res.push(get_string_from_hm(hm, &char1));
                char1 = c.to_string();
            } else {
                char1 += &c.to_string();
            }
            is_digit = false
        }
    }
    if is_digit {
        res.push(get_string_from_hm(hm, &char1));
    } else {
        res.push(char1);
    }

    return res.join("");
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
pub fn read_write(f1: &str, f2: &str, hm: &HashMap<&str, usize>, count: &usize) {
    let file = File::open(f1).unwrap();
    let mut reader = BufReader::new(file);
    let file = File::create(f2).unwrap();
    let mut writer = std::io::BufWriter::new(file);
    let mut c = 0;
    let mut lastpro = 0.0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut fields: Vec<&str> = line.split_whitespace().collect();
        match fields[0]  {
            "S" => {
                let a = convert_string(fields[1], hm);
                fields[1] = &a;
                writeln!(writer, "{}", fields.join("\t")).expect("Error writing to file");
            },
            "L" => {
                let a = convert_string(fields[1], hm);
                fields[1] = &a;
                let b = convert_string(fields[3], hm);
                fields[3] = &b;
                writeln!(writer, "{}", fields.join("\t")).expect("Error writing to file");
            },
            "P" => {
                let a = convert_string(fields[2], hm);
                fields[2] = &a;
                writeln!(writer, "{}", fields.join("\t")).expect("Error writing to file");
            }
            "J" => {
                let a = convert_string(fields[1], hm);
                fields[1] = &a;
                let b = convert_string(fields[3], hm);
                fields[3] = &b;
                writeln!(writer, "{}", fields.join("\t")).expect("Error writing to file");
            },
            "W" => {
                let a = convert_string(fields[6], hm);
                fields[6] = &a;
                writeln!(writer, "{}", fields.join("\t")).expect("Error writing to file");
            },
            "C" => {
                let a = convert_string(fields[1], hm);
                fields[1] = &a;
                let b = convert_string(fields[3], hm);
                fields[3] = &b;
                writeln!(writer, "{}", fields.join("\t")).expect("Error writing to file");
            }
            "F" => {
                let a = convert_string(fields[1], hm);
                fields[1] = &a;
                writeln!(writer, "{}", fields.join("\t")).expect("Error writing to file");
            }
            "E" => {
                let a = convert_string(fields[2], hm);
                fields[2] = &a;
                let n = convert_string(fields[3], hm);
                fields[3] = &a;
                writeln!(writer, "{}", fields.join("\t")).expect("Error writing to file");
            }
            "G" => {
                let a = convert_string(fields[2], hm);
                fields[2] = &a;
                let n = convert_string(fields[3], hm);
                fields[3] = &a;
                writeln!(writer, "{}", fields.join("\t")).expect("Error writing to file");
            }
            "U" => {
                let mut b: Vec<String> = vec![fields[0].to_string(), fields[1].to_string()];
                for x in fields.iter().skip(2) {
                    let a = convert_string(x, hm);
                    b.push(a);
                }
                writeln!(writer, "{}", b.join("\t")).expect("Error writing to file");
            }
            "O" => {
                let mut b: Vec<String> = vec![fields[0].to_string(), fields[1].to_string()];
                for x in fields.iter().skip(2) {
                    let a = convert_string(x, hm);
                    b.push(a);
                }
                writeln!(writer, "{}", b.join("\t")).expect("Error writing to file");
            }


            _ => writeln!(writer, "{}", line).expect("Error writing to file"),
        }
        c += 1;
        if  (c as f64 / *count as f64) * 100.0 - lastpro > 0.2 {
            lastpro = (c as f64 / *count as f64) * 100.0;
            eprint!("\r{}", format!("{} - Progress {:.2}%", Local::now().format("%d/%m/%Y %H:%M:%S %p"), lastpro));
            io::stdout().flush().expect("Failed to flush stdout");
        }

    }
    eprintln!("\r{}", format!("{} - Progress {:.2}%", Local::now().format("%d/%m/%Y %H:%M:%S %p"), 100.0));

}


/// Get the version of a GFA file
/// Not sure if the header line must be the first line in the file
/// Return:
///     - f32: version number
pub fn get_version(filename: &str) -> f32{
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut version = 0.0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut fields: Vec<&str> = line.split_whitespace().collect();
        if fields[0] == "H" {
            version = fields[1].split(":").collect::<Vec<&str>>()[2].parse::<f32>().expect("Error parsing version number")
        }
    }
    version
}

/// Write a hashmap to a file
///
/// Return:
///     - ()
///     - Tab separated file with key and value
pub fn write_hm(hm: &HashMap<&str, usize>, f: &str) {
    let file = File::create(f).unwrap();
    let mut writer = std::io::BufWriter::new(file);
    for (k, v) in hm.iter() {
        writeln!(writer, "{}\t{}", k, v).expect("Error writing to file");
    }
}


