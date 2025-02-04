// Implement all the necessary pkgs

use std::f64;
use std::iter::repeat_with;

// Helper function to compute the mean of a vector
fn mean(data: &[f64]) -> f64{
    data.iter().sum::<f64>() / data.len() as f64
}

// Helper function to compute the covariance between two vector x and y
fn covariance(x: &[f64], y: &[f64], mean_x: f64, mean_y: f64) -> f64{
    let n = x.len();
    x.iter().zip(y.iter()).map(|(xi, yi)| (xi - mean_x) *(yi - mean_y) ).sum::<f64>() / (n as f64 - 1.0)
}

// Helper function: Computing the autocorrelation at lag 'p'
fn autocorrelation(data: &[f64], p: usize) -> f64{
    if p >= data.len(){
        return 0.0;
    }
    let mean_val = mean(data);
    covariance(&data[0..data.len() - p], &data[p..], mean_val, mean_val) / covariance(data, data, mean_val, mean_val)
}

// Function to compute the coefficient - Fit function
pub fn fit_ar_model(data: &[f64], p: usize) -> Vec<f64>{
    let mut autocorr = vec![0.0; p];
    for lag in 1..=p{
        autocorr[lag - 1] = autocorrelation(data, p);
    }

    let mut coefficients = vec![0.0; p];
    for i in 0..p{
        coefficients[i] = autocorr[i];
    }
    return coefficients;
}

// Predict function
pub fn predict_ar_model(data: &[f64], p: usize, coeffs: &[f64]) -> Vec<f64>{
    let mut predictions = Vec::new();
    for t in p..data.len(){
        let mut prediction = 0.0;
        for i in 0..p{
            prediction += coeffs[i] * data[t-1-i];
        }
        predictions.push(prediction)
    }
    return predictions
}

// #[cfg(test)]
// mod tests{
//     use super::*;
//     #[test]
//     fn test_ar_model(){
//         let data: f64 = [1.0,1.25,1.50,2.25,2.75,3.75,4.15,5.25];
//         let p: usize = 5;
        
//         // Mean Test
//         fn test_mean(){
//             assert_eq!(mean(&data), 3.0);
//         }

//         // Test the autocorrelation
//         fn test_autocorrelation(){
//             let ac = autocorrelation(&data, lag);
//             assert!(ac > 0.0); //Positive correlation
//         }

//         // Test fit_ar_model
//         fn test_fit_ar_model(){
//             let coeffs = fit_ar_model(&data, p);
//             assert_eq!(coeffs.len(), p);
//         }

//         // Test prediction
//         fn test_ar_predict(){
//             let coeffs = vec![0.5, 0.2];
//             let predictions = predict_ar_model(&data, p, &coeffs);
//             assert_eq!(predictions.len(), data.len() - p);
//         }
//     }
// }

