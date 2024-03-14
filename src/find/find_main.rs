use clap::ArgMatches;
use gfa_reader::NCGfa;

use std::cmp::max;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub fn find_main(matches: &ArgMatches) {
    println!("Starting find_main");
    let graph_file = matches.value_of("gfa").unwrap();
    let feature_file = matches.value_of("features").unwrap();
    let output = matches.value_of("output").unwrap();
    let length = matches.value_of("length").unwrap().parse::<i128>().unwrap();
    let data = FileData::from_file(feature_file);
    let feature = data.feature;
    // Read the graph
    let mut graph: NCGfa<()> = NCGfa::new();
    graph.parse_gfa_file(graph_file, false);
    let paths = &graph.paths;

    // Get the node size
    let node_size = node_size(&graph);

    //
    let mut position_nodesize = Vec::new();
    let mut vec_res_u64 = Vec::new();

    for path in paths.iter() {
        let mut vec_u64 = Vec::new();
        let mut index = Vec::new();
        let mut pos = 0;
        for i in 0..path.nodes.len() - 1 {
            index.push([pos, node_size[path.nodes[i] as usize - 1]]);
            pos += node_size[path.nodes[i] as usize - 1];
            let v1 = path.nodes[i];
            let v2 = path.dir[i];
            let v3 = path.nodes[i + 1];
            let v4 = path.dir[i + 1];

            if feature == Feature::Node {
                vec_u64.push(v1 as u64);
            } else if feature == Feature::DirNode {
                vec_u64.push(v1 as u64 * 2 + v2 as u64);
            } else if feature == Feature::Edge {
                let u1 = v1 * 2 + v2 as u32;
                let u2 = v3 * 2 + v4 as u32;
                vec_u64.push(merge_u32_to_u64(u1, u2));
            }
        }
        vec_res_u64.push(vec_u64);
        position_nodesize.push(index)
    }

    let file = File::create(output).unwrap();
    let mut writer = std::io::BufWriter::new(file);
    let data_hs = data.data.iter().collect::<HashSet<&u64>>();
    for (i, x) in vec_res_u64.iter().enumerate() {
        for (i2, y) in x.iter().enumerate() {
            if data_hs.contains(y) {
                writeln!(
                    writer,
                    "{}\t{}\t{}\tID:{};NS:{};NB:{}",
                    graph.paths[i].name,
                    max(0, position_nodesize[i][i2][0] as i128 - length),
                    position_nodesize[i][i2][0] as i128
                        + position_nodesize[i][i2][1] as i128
                        + 1
                        + length,
                    to_string1(*y, &feature),
                    position_nodesize[i][i2][1],
                    position_nodesize[i][i2][0],
                )
                .expect("Error writing to file")
            }
        }
    }
}

// Size of each node
pub fn node_size(graph: &NCGfa<()>) -> Vec<usize> {
    let res = graph
        .nodes
        .iter()
        .map(|x| x.seq.len())
        .collect::<Vec<usize>>();
    res
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub enum Feature {
    Node,
    DirNode,
    Edge,
    Alignment,
}

impl Feature {
    pub fn from_str(s: &str) -> Self {
        match s {
            "node" => Feature::Node,
            "dirnode" => Feature::DirNode,
            "edge" => Feature::Edge,
            _ => panic!("Not implemented"),
        }
    }

    pub fn to_string1(&self) -> String {
        match self {
            Feature::Node => "node".to_string(),
            Feature::DirNode => "dirnode".to_string(),
            Feature::Edge => "edge".to_string(),
            Feature::Alignment => "alignment".to_string(),
        }
    }
}

pub fn from_string(name_input: &str, ftype: Feature) -> u64 {
    if ftype == Feature::Node {
        name_input.parse().unwrap()
    } else if ftype == Feature::DirNode {
        let last_char = &name_input[name_input.len() - 1..];
        let rest = &name_input[..name_input.len() - 1];

        rest.parse::<u64>().unwrap() * 2 + (last_char == "+") as u64
    } else {
        let ff = name_input.find(|c| c == '+' || c == '-').unwrap();
        let dir1 = &name_input[ff..ff];
        let dir2 = &name_input[name_input.len() - 1..name_input.len() - 1];

        let number1 = &name_input[..ff];
        let number2 = &name_input[ff + 1..];

        let numb1: u32 = number1.parse::<u32>().unwrap() * 2 + (dir1 == "+") as u32;
        let numb2: u32 = number2.parse::<u32>().unwrap() * 2 + (dir2 == "+") as u32;

        merge_u32_to_u64(numb1, numb2)
    }
}

pub fn to_string1(input: u64, ftype: &Feature) -> String {
    if Feature::Node == *ftype {
        input.to_owned().to_string()
    } else if *ftype == Feature::DirNode {
        return format_unsigned_as_string(input);
    } else if *ftype == Feature::Edge {
        let (left, right) = split_u64_to_u32s(input);

        return format_unsigned_as_string(left) + &format_unsigned_as_string(right);
    } else {
        let (left, right) = split_u64_to_u32s(input);
        return left.to_string() + "_" + right.to_string().as_str();
    }
}

pub fn merge_u32_to_u64(high: u32, low: u32) -> u64 {
    let high_u64 = u64::from(high);
    let low_u64 = u64::from(low);

    let result: u64 = (high_u64 << 32) | low_u64;

    result
}

pub fn split_u64_to_u32s(value: u64) -> (u32, u32) {
    let low = value as u32;
    let high = (value >> 32) as u32;

    (high, low)
}

fn format_unsigned_as_string<T: Display + Into<u64>>(name: T) -> String {
    let name_u64 = name.into();
    format!(
        "{}{}",
        name_u64 / 2,
        if name_u64 % 2 == 1 { "+" } else { "-" }
    )
}

pub struct FileData {
    pub data: Vec<u64>,
    pub feature: Feature,
}

impl FileData {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            feature: Feature::Node,
        }
    }

    pub fn from_file(filename: &str) -> Self {
        let feature = get_type(filename);

        let mut data = Vec::new();
        let file = File::open(filename).expect("ERROR: CAN NOT READ FILE\n");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            if feature == Feature::Edge {
                let ss = find_first_plus_minus(&line).unwrap();
                let s1 = &line[..ss];
                let s2 = &line[ss..ss + 1];
                let s3 = &line[ss + 1..line.len() - 1];
                let s4 = &line.chars().last().unwrap();
                let ss1 = s1.parse::<u64>().unwrap() * 2 + (s2 == "+") as u64;
                let ss2 = s3.parse::<u64>().unwrap() * 2 + (*s4 == '+') as u64;
                data.push(merge_u32_to_u64(ss1 as u32, ss2 as u32));
            } else if feature == Feature::DirNode {
                let s = line.ends_with('+');
                let s2 = line[..line.len() - 1].parse::<u64>().unwrap() * 2 + s as u64;
                data.push(s2)
            } else {
                data.push(line.parse::<u64>().unwrap());
            }
        }
        // Sort very important
        data.sort();
        Self { data, feature }
    }
}

fn find_first_plus_minus(input: &str) -> Option<usize> {
    input.chars().position(|c| c == '+' || c == '-')
}

pub fn get_type(file_path: &str) -> Feature {
    let file = File::open(file_path).expect("ERROR: CAN NOT READ FILE\n");

    // Parse plain text or gzipped file
    let reader = BufReader::new(file);

    // Read the first line of the file
    let first_line = reader.lines().next().unwrap().unwrap();
    let parts: Vec<&str> = first_line
        .split(|c| c == '+' || c == '-')
        .filter(|s| !s.is_empty()) // Filter out empty strings
        .collect();
    println!("parts: {:?}", parts);
    let last_letter = first_line.chars().last().unwrap();
    if last_letter == '+' || last_letter == '-' {
        if parts.len() == 1 {
            Feature::DirNode
        } else {
            Feature::Edge
        }
    } else {
        Feature::Node
    }
}
