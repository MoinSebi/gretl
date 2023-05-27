use std::fmt::Debug;

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
