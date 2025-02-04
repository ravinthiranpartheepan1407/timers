// We are gonna use seasonal differencing approach: Subtract each value x[i] - x[i-1] (Current - previous season) 
// Threshold = [days, weeks, months, years]
// Implement some conditions to check if the differenced series shows stationarity beahvior (Ued for checking the indication of seasonality)


// Necessary lib import
use std::f64;

// Mean
fn mean(data: &[f64]) -> f64{
    let sum: f64 = data.iter().sum();
    sum / data.len() as f64
}

// Seasonal Differencing
fn seasonal_difference(data: &[f64], period: usize) -> Vec<f64>{
    let mut difference = Vec::new();
    for elements in period..data.len(){
        difference.push(data[elements] - data[elements - period]);
    }
    return difference
}

// Std-Dev
fn std_dev(data: &[f64], mean: f64) -> f64{
    let std_dev: f64 = data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64;
    std_dev.sqrt()
}

// Check_seasonality
pub fn check_seasonality(data: &[f64], period: usize) -> bool{
    // Perform the seasonal differencing
    let differenced = seasonal_difference(data, period);

    // Check if the differenced data is stationary by comparing the rolling mean and rolling std dev
    let mean_val = mean(&differenced);
    let std_dev_val = std_dev(&differenced, mean_val);

    // If the std_dev of the differenced series is low, then we can say seasonality is present
    return std_dev_val > 0.05 * mean_val;
}

