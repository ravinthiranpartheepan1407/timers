// Import necessary pkg
use std::f64;

pub fn simple_moving_average(data: &[f64], window_size: usize) -> Vec<f64>{
    let mut result = Vec::new();
    if data.len() < window_size{
        return result; //Empty vector
    }

    for i in 0..=data.len() - window_size{
        let window = &data[i..i + window_size]; // [0,1,2] -> [1,2,3] -> ...
        let sum: f64 = window.iter().sum();
        let avg = sum / window_size as f64;
        result.push(avg)
    }

    return result;
}