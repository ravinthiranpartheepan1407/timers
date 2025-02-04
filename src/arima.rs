use std::f64;

// key and value pair data
pub struct ARIMA{
    p: usize,
    q: usize,
    d: usize,
    ar_coeffs: Vec<f64>,
    ma_coeffs: Vec<f64>,
    differenced_series: Vec<f64>,
}

// oop -> we can access the shared methods using inheritance
//  But i Rust we can access the shared methods using traits

impl ARIMA{
    pub fn new(p: usize, d: usize, q: usize, ar_coeffs: Vec<f64>, ma_coeffs: Vec<f64>) -> Self{
        ARIMA{
            p, 
            q,
            d,
            ar_coeffs,
            ma_coeffs,
            differenced_series: Vec::new(),
        }
    }

    // Another function for difference
    pub fn difference(&mut self, series: &[f64]) -> Vec<f64>{
        let mut differenced = series.to_vec();
        for _ in 0..self.d{
            differenced = differenced.windows(2).map(|w| w[1] - w[0]).collect();
        }
        self.differenced_series = differenced.clone();
        return differenced
    }

    // function for calculating the inverse difference
    pub fn inverse_difference(&self, forecast: f64, original_series: &[f64]) -> f64{
        let last_original = original_series[original_series.len() - self.d];
        return last_original + forecast
    }

    // fn prredict
    pub fn predict(&self, series: &[f64]) -> f64{
        let diff_series = self.differenced_series.clone();
        let mut ar_term = 0.0;
        let mut ma_term = 0.0;
        let noise = 0.0;

        // Compute AR term
        if self.p > 0{
            let p_values: Vec<f64> = diff_series.iter().rev().take(self.p).cloned().collect();
            for(i, &phi) in self.ar_coeffs.iter().enumerate(){
                if i < p_values.len(){
                    ar_term += phi * p_values[i];
                }
            }
        }

        // Compute MA term
        if self.q > 0 {
            let q_values: Vec<f64> = diff_series.iter().rev().take(self.q).cloned().collect();
            for(i, &theta) in self.ma_coeffs.iter().enumerate(){
                if i < q_values.len(){
                    ma_term += theta * noise;
                }
            }
        }

        let forecast = ar_term + ma_term + noise;
        return self.inverse_difference(forecast, series);
    }
}