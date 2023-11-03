use rand::{seq::IteratorRandom, thread_rng}; // 0.6.1


/// Create a a vector of n (number) random numbers
/// size = Random number between 0 and size
/// number = amount of numbers you want to draw
pub fn random_numbers(size: &usize, number: &usize) -> Vec<usize>{
    let mut rng = thread_rng();
    let v: Vec<usize> = (0..*size).into_iter().collect();
    let sample: Vec<&usize>= v.iter().choose_multiple(&mut rng, *number);

    // Clone (so no reference)
    let sample: Vec<usize> = sample.iter().map(|n| *n).cloned().collect();
    sample
}