// Mean
// Variance
// Autocorrelation
//  Check white noise:
// mean < 0.05 
// variance > 0.05 and auto correlation at given lap < 0.05

use std::f64;

fn mean(data: &[f64]) -> f64{
    let sum: f64 = data.iter().sum();
    sum / data.len() as f64
}

fn variance(data: &[f64], mean: f64) -> f64{
    let variance: f64 = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64;
    variance
}

fn autocorrelation(data: &[f64], lag: usize) -> f64{
    let n = data.len();
    if lag >= n{
        panic!("Lag must be less than the lenght of the data array")
    }

    let mean_val = mean(data);
    let mut numerator = 0.0;
    let mut denominator = 0.0;

    for elements in 0..(n - lag){
        numerator += (data[elements] - mean_val) * (data[elements + lag] - mean_val);
    }

    for elements in 0..n{
        denominator += (data[elements] - mean_val).powi(2);
    }

    numerator / denominator
}

pub fn check_white_noise(data: &[f64], lag: usize) -> bool{

    // Check if the mean is closer to zero
    let mean_val = mean(data);
    if mean_val.abs() > 0.05{
        println!("The given time series mean is too far away from the threshold 0: {}", mean_val);
        return false;
    }

    // Check if the variance is close to the constant
    let var_val = variance(data, mean_val);
    if var_val < 0.05 {
        println!("The variance is too low: {}", var_val);
        return false;
    }

    // Check if the autocorrelation at specified lag is closer to zero
    let autocorr = autocorrelation(data, lag);
    if autocorr.abs() > 0.05{
        println!("Autocorrelation at lag {} is higher: {}", lag, autocorr);
        return false;
    }

    return true;

}