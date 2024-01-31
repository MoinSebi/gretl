/// Calculate average of the vector
pub fn mean321(data: &[f64]) -> f64 {
    let sums1: f64 = data.iter().sum();
    let sums = sums1 / data.iter().len() as f64;
    sums
}

pub fn mean(data: &[u32]) -> f64 {
    let sums1: u32 = data.iter().sum();
    let sums = (sums1 as f64) / data.iter().len() as f64;
    sums
}
pub fn mean_usize(data: &[usize]) -> f64 {
    let sums1: usize = data.iter().sum();
    let sums = (sums1 as f64) / data.iter().len() as f64;
    sums
}

/// Calculate median of the vector
pub fn median(data: &Vec<u32>) -> f64 {
    data[data.len() / 2] as f64
}

pub fn median2(data: &Vec<usize>) -> usize {
    data[data.len() / 2]
}

// standard deviation of a usize vector
pub fn std_usize(data: &Vec<usize>) -> f64 {
    let mean = mean_usize(data);
    let mut sum = 0.0;
    // This is a alternative approach
    for i in data.iter() {
        sum += (*i as f64 - mean).powf(2.0);
    }
    
    (sum / (data.len() as f64)).sqrt()
}

/// Standard deviation function
pub fn stadard_deviation(data: &[u32]) -> f64 {
    let mean = mean(data);
    let mut sum = 0.0;
    // This is a alternative approach
    for i in data.iter() {
        sum += (*i as f64 - mean).powf(2.0);
    }
    
    (sum / (data.len() as f64)).sqrt()
}

pub fn stadard_deviation_2(data: &[f64]) -> f64 {
    let mean = mean321(data);
    let mut sum = 0.0;
    // This is a alternative approach
    for i in data.iter() {
        sum += (*i - mean).powf(2.0);
    }
    
    (sum / (data.len() as f64)).sqrt()
}

/// Combi function for average, median and std
pub fn average_median_std(vec_size: &mut Vec<u32>) -> (f64, f64, f64) {
    vec_size.sort();
    let med = median(vec_size);
    let average = mean(vec_size);
    let std = stadard_deviation(vec_size);

    (average, med, std)
}
