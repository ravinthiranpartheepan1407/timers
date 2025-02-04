// Import necessary modules

use std::f64;

fn mean(data: &[f64]) -> f64{
    // x[0]+x[1]+...[xn]
    let sum: f64 = data.iter().sum();
    sum / data.len() as f64
}

fn std_dev(data: &[f64], mean: f64) -> f64{
    // sigma = sqrt( Nominator: (sum (xi - mean)^2) / n: data.len )
    // (x[0] - mean)^2 + (x[1] - mean)^2 +. .... +  
    let std_dev = data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64;
    std_dev.sqrt()
}

// Function to check the stationarity by comapring the rolling mean and rolling std_dev

pub fn check_stationarity(data: &[f64], window_size: usize) -> bool{
    if window_size > data.len(){
        panic!("Window size must be less than or equal to the length of the given data");
    }

    let mut prev_mean = mean(&data[0..window_size]);
    let mut prev_std_dev = std_dev(&data[0..window_size], prev_mean);

    for elements in window_size..data.len(){
        let window = &data[elements - window_size..elements];
        let current_mean = mean(window);
        let current_std_dev = std_dev(window, current_mean);

        // Compare the rolling mean and rolling std_dev
        if(current_mean - prev_mean).abs() > 0.05 * prev_mean || (current_std_dev - prev_std_dev).abs() > 0.05 * prev_std_dev{
            return false;
            // Time series is like non-stationary
        }

        prev_mean = current_mean;
        prev_std_dev = current_std_dev;
    }
    return true;
}

// #[cfg(test)]
// mod tests{
//     use super::*;
//     #[test]
//     fn test_stationarity(){
//         let data = [1.0,2.0,3.0,4.0,5.0];
//         let result = check_stationarity(&data, 2);
//         assert!((result - 0.5).abs() < 1e-10);
//     }
// }