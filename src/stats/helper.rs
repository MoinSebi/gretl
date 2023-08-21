use std::collections::{HashMap, HashSet};
use gfa_reader::{Gfa, GraphWrapper, NCEdge, NCGfa, NCPath};



/// Calculate average of the vector
pub fn mean(data: & [u32]) -> f64{
    let sums1: u32 = data.iter().sum();
    let sums = (sums1 as f64)/data.iter().len() as f64;
    return sums
}


/// Calculate average of the vector
pub fn meanf(data: & [f32]) -> f64{
    let sums1: f32 = data.iter().sum();
    let sums = (sums1 as f64)/data.iter().len() as f64;
    return sums
}

/// Calculate median of the vector
pub fn median(data: &mut Vec<u32>) -> f64{
    data.sort();
    return data[data.len()/2] as f64
}

/// Get the file name
///
/// Remove folder structure
///
pub fn get_filename(name: &str) -> String{
    let u: Vec<&str> = name.split("/").collect();
    u.last().unwrap().to_string()

}

pub fn std(data: & [u32]) -> f64{
    let mean = mean(data);
    let mut sum = 0.0;
    for i in data.iter(){
        sum += (*i as f64 - mean).powf(2.0);
    }
    let std = (sum/(data.len() as f64)).sqrt();
    return std
}


