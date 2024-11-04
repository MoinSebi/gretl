use gfa_reader::{Gfa, Path};
use log::info;
use std::fmt::Debug;
use std::fs::File;

#[allow(dead_code)]
/// Calculate the average
fn calculate_average<T>(v: &[T]) -> Option<f64>
where
    T: Into<f64> + Copy + Debug,
{
    if v.is_empty() {
        return None;
    }

    let mut mean: f64 = f64::from(0);
    let mut count: f64 = 0.0;

    for &value in v {
        mean += (value.into() - mean) / (count + 1.0);
        count += 1.0;
    }

    Some(mean)
}

/// Counting the amount of accessions and depth
pub fn calc_depth(
    wrapper: &Vec<(String, Vec<&Path<u32, (), ()>>)>,
    graph: &Gfa<u32, (), ()>,
) -> Vec<u32> {
    let mut depth: Vec<u32> = vec![0; graph.segments.iter().max().unwrap().id as usize + 1];
    for paths in wrapper.iter() {
        for path in paths.1.iter() {
            for x in path.nodes.iter() {
                depth[*x as usize] += 1;
            }
        }
    }
    depth.shrink_to_fit();
    depth
}

pub fn calc_similarity(
    wrapper: &Vec<(String, Vec<&Path<u32, (), ()>>)>,
    graph: &Gfa<u32, (), ()>,
) -> Vec<u32> {
    let mut depth: Vec<u32> = vec![0; graph.segments.iter().max().unwrap().id as usize + 1];
    for p in wrapper.iter() {
        let mut path_nodes: Vec<u32> = p.1.iter().flat_map(|x| x.nodes.iter()).cloned().collect();
        path_nodes.sort();
        path_nodes.dedup();
        for x in path_nodes.iter() {
            depth[*x as usize] += 1;
        }
    }
    depth.shrink_to_fit();
    depth
}

/// Calculate node degree (in, out, total)
pub fn calc_node_degree(graph: &Gfa<u32, (), ()>) -> (Vec<u32>, Vec<u32>, Vec<u32>) {
    let mut degree_in: Vec<u32> = vec![0; graph.segments.iter().max().unwrap().id as usize + 1];
    let mut degree_out: Vec<u32> = vec![0; graph.segments.iter().max().unwrap().id as usize + 1];
    let mut degree_total: Vec<u32> = vec![0; graph.segments.iter().max().unwrap().id as usize + 1];
    for x in graph.links.iter() {
        let fromu: usize = x.from as usize;
        let tou: usize = x.to as usize;

        degree_out[fromu] += 1;
        degree_in[tou] += 1;
        degree_total[fromu] += 1;
        degree_total[tou] += 1;
    }
    (degree_in, degree_out, degree_total)
}

/// Compute the node len
///
/// Return a vector
pub fn calc_node_len(graph: &Gfa<u32, (), ()>) -> Vec<u32> {
    let mut result = vec![0; graph.segments.iter().max().unwrap().id as usize + 1];

    for node in graph.segments.iter() {
        result[node.id as usize] = node.sequence.get_len() as u32;
    }
    result
}

pub fn mean<T>(data: &[T]) -> f64
where
    T: Into<f64> + Copy,
{
    if data.is_empty() {
        return f64::NAN;
    }

    let sum: f64 = data.iter().map(|&x| x.into()).sum();
    sum / (data.len() as f64)
}

pub fn median<T>(data: &[T]) -> f64
where
    T: PartialOrd + Into<f64> + Copy,
{
    let mut sorted_data: Vec<T> = data.to_vec();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let len = sorted_data.len();
    if len == 0 {
        return f64::NAN;
    }

    if len % 2 == 0 {
        // Average of the two middle numbers if even length
        let mid1 = sorted_data[len / 2 - 1];
        let mid2 = sorted_data[len / 2];
        (mid1.into() + mid2.into()) / 2.0
    } else {
        // Return the middle number if odd length
        sorted_data[len / 2].into()
    }
}

pub fn standard_deviation<T>(data: &[T], mean: f64) -> f64
where
    T: Into<f64> + Copy,
{
    let n = data.len();
    if n == 0 {
        return f64::NAN; // Standard deviation is undefined for empty input
    }

    // Sum of squared differences from the mean
    let sum_of_squares: f64 = data
        .iter()
        .map(|&x| {
            let val = x.into();
            let diff = val - mean;
            diff * diff
        })
        .sum();

    // Calculate variance and standard deviation
    let variance = sum_of_squares / (n as f64);
    variance.sqrt()
}

/// Combi function for average, median and std
pub fn average_median_std<T>(vec_size: &Vec<T>) -> (f64, f64, f64)
where
    T: Into<f64> + Copy + PartialOrd + Ord + Default,
{
    let mut vec_size = vec_size.clone();
    vec_size.sort();
    vec_size.retain(|&x| x != T::default());

    let med = median(&vec_size);
    let average = mean(&vec_size);
    let std = standard_deviation(&vec_size, average);

    (average, med, std)
}

use std::io::{stdout, BufWriter, Write};

/// Prints or writes the formatted string depending on the output provided
/// If `output` is Some, it writes to the given file, otherwise it prints to stdout.
pub fn print_or_write(output: Option<&mut dyn Write>, message: &str) {
    match output {
        Some(mut output) => {
            writeln!(output, "{}", message).expect("Failed to write to output");
        }
        None => {
            writeln!(stdout(), "{}", message).expect("Failed to write to output");
        }
    }
}

use std::io::{self};

pub fn get_writer(filename: &str) -> io::Result<Box<dyn Write>> {
    if filename == "-" || filename.is_empty() {
        // Create a BufWriter for stdout
        let stdout = io::stdout();
        let writer = BufWriter::new(stdout);
        Ok(Box::new(writer))
    } else {
        // Create a BufWriter for a file
        let file = File::create(filename)?;
        let writer = BufWriter::new(file);
        Ok(Box::new(writer))
    }
}
