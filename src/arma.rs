use std::f64;

pub fn arma(ar_coeffs: &[f64], ma_coeffs: &[f64], c: f64, n: usize) -> Vec<f64>{
    let mut y_values = vec![0.0; n];
    let mut e_values = vec![0.0; n];

    // We are generating simple determinstic random white noise using sin()
    for t in 0..n{
        e_values[t] = (t as f64).sin() * 0.1;
    }

    for t in 0..n{
         // Auto-regressive Component: Worst time complexity
        let mut ar_term = 0.0;
        for(i, &phi) in ar_coeffs.iter().enumerate(){
            if t >= i  + 1{
                ar_term += phi * y_values[t - i - 1];
            }
        }
        // Moving Average Component
        let mut ma_term = 0.0;
        for(j, &theta) in ma_coeffs.iter().enumerate(){
            if t >= j + 1{
                ma_term += theta * e_values[t - j - 1];
            }
        }

        y_values[t] = c + ar_term + ma_term + e_values[t];
    }
    return y_values;
}