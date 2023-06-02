use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use gfa_reader::Gfa;

fn calculate_average<T>(v: &[T]) -> Option<f64>
    where T:  Into<f64> + Copy + Debug
{
    if v.is_empty() {
        return None;
    }

    let mut mean: f64 = f64::from(0);
    let mut count: f64 = 0.0;

    for &value in v {
        mean += ((value.into() - mean)/ f64::from(count + 1.0));
        count += 1.0;
    }

    Some(mean)
}


pub fn core1(graph: &Gfa) -> HashMap<&String, u32>{
    let mut count: HashMap<&String, u32> = HashMap::new();
    for x in &graph.nodes{
        count.insert(x.0, 0);
    }

    for p in graph.paths.iter(){
        let v: HashSet<_> = p.nodes.iter().cloned().collect();
        for y in v.iter(){
            *count.get_mut(y).unwrap() += 1;
        }
    }
    count
}
