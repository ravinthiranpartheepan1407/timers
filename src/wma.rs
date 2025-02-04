use std::f64;

pub fn weighted_moving_average(data: &[f64], window_size: usize, weights: &[f64]) -> Vec<f64>{
    let mut result = Vec::new();
    if data.len() < window_size{
        return result;
    }

    for i in 0..=data.len() - window_size{
        let window = &data[i..i + window_size];
        let weighted_sum: f64 = window.iter().enumerate().map(|(j, &value)| value * weights[j]).sum();
        let sum_of_weights: f64 = weights.iter().sum();
        let weighted_avg = weighted_sum / sum_of_weights;
        result.push(weighted_avg);
    }

    return result;
}