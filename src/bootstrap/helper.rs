use rand::{seq::IteratorRandom, thread_rng}; // 0.6.1


pub fn random_numbers(size: &usize, number: &usize) -> Vec<usize>{
    let mut rng = thread_rng();
    let v: Vec<usize> = (0..*size).into_iter().collect();
    let sample: Vec<&usize>= v.iter().choose_multiple(&mut rng, *number);
    println!("{}", sample.len());
    println!("t{}", number);
    let sample: Vec<usize> = sample.iter().map(|n| n.clone()).cloned().collect();
    sample

}