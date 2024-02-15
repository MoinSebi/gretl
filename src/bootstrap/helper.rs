use rand::{seq::IteratorRandom, thread_rng}; // 0.6.1
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;


/// Create a a vector of n (number) random numbers
/// size = Random number between 0 and size
/// number = amount of numbers you want to draw
pub fn random_numbers(size: &usize, number: &usize) -> Vec<usize> {
    let mut rng = thread_rng();
    let v: Vec<usize> = (0..*size).collect();
    let sample: Vec<&usize> = v.iter().choose_multiple(&mut rng, *number);

    // Clone (so no reference)
    let sample: Vec<usize> = sample.iter().copied().cloned().collect();
    sample
}

pub fn read_positive_integers_from_file(filename: &str) -> Vec<u32> {
    let file = File::open(filename).expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let mut integers = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let number: u32 = line.trim().parse().expect("Failed to parse number");
        integers.push(number);
    }

    integers
}