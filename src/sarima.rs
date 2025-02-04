use std::f64;


pub struct SARIMA{
    p: usize,
    q: usize,
    d: usize,
    P: usize,
    Q: usize,
    D: usize,
    m: usize, //Seasonal cycle length
    ar_coeffs: Vec<f64>,
    ma_coeffs: Vec<f64>,
    seasonal_ar_coeffs: Vec<f64>,
    seasonal_ma_coeffs: Vec<f64>,
    differenced_series: Vec<f64>,
}

impl SARIMA{
    // Constructor for SARIMA
    pub fn new(p: usize, d: usize, q: usize, P: usize, Q: usize, D: usize, m: usize, ar_coeffs: Vec<f64>, ma_coeffs: Vec<f64>, seasonal_ar_coeffs: Vec<f64>, seasonal_ma_coeffs: Vec<f64>) -> Self{
        SARIMA{
            p, d, q, P, Q, D, m, ar_coeffs, ma_coeffs, seasonal_ar_coeffs, seasonal_ma_coeffs, differenced_series: Vec::new()
        }
    }

    // Differencing func -> removing if there is a trend or seasonality in the data
    pub fn difference(&mut self, series: &[f64]) -> Vec<f64>{
        let mut differenced = series.to_vec();

        // Perform the non-seasonal differencing
        for _ in 0..self.d{
            differenced = differenced.windows(2).map(|w| w[1] - w[0]).collect()
        }

        // Perform the seasonal differencing
        for _ in 0..self.D{
            if differenced.len() > self.m{
                differenced = differenced.iter().skip(self.m).zip(differenced.iter()).map(|(a, b)| b - a).collect();
            }
        }

        self.differenced_series = differenced.clone();
        return differenced;
    }

    // Inverse differencing to recover the original series
    pub fn inverse_difference(&self, forecast: f64, original_series: &[f64]) -> f64{
        let last_original = original_series[original_series.len() - self.d];
        return last_original + forecast;
    }

    // Predict - SARIMA: AR + MA + Seasonal Components
    pub fn predict(&self, series: &[f64]) -> f64{
        let diff_series = self.differenced_series.clone();
        let mut ar_term = 0.0;
        let mut ma_term = 0.0;
        let mut seasonal_ar_term = 0.0;
        let mut seasonal_ma_term = 0.0;
        let noise = 0.0;  

        // Compute the AR model
        if self.p > 0{
            let p_values: Vec<f64> = diff_series.iter().rev().take(self.p).cloned().collect();
            for(i, &phi) in self.ar_coeffs.iter().enumerate(){
                if i < p_values.len(){
                    ar_term += phi + p_values[i]
                }
            }
        }

        // Compute the MA term
        if self.q > 0{
            let q_values: Vec<f64> = diff_series.iter().rev().take(self.q).cloned().collect();
            for(i, &theta) in self.ma_coeffs.iter().enumerate(){
                if i < q_values.len(){
                    ma_term += theta * noise;
                }
            }
        }

        // We are gonna compute the seasonality AR term
        if self.P > 0{
            let seasonal_P_values: Vec<f64> = diff_series.iter().rev().take(self.P * self.m).step_by(self.m).cloned().collect();
            for(i, &phi_s) in self.seasonal_ar_coeffs.iter().enumerate(){
                if i < seasonal_P_values.len(){
                    seasonal_ar_term += phi_s * seasonal_P_values[i];
                }
            }
        }

        // Same for the seasonality of MA term
        if self.Q > 0{
            let seasonal_Q_values: Vec<f64> = diff_series.iter().rev().take(self.Q * self.m).step_by(self.m).cloned().collect();
            for(i, &theta_s) in self.seasonal_ma_coeffs.iter().enumerate(){
                if i < seasonal_Q_values.len(){
                    seasonal_ma_term += theta_s * noise;
                }
            }
        }

        let forecast = ar_term + ma_term + seasonal_ar_term + seasonal_ma_term;
        return self.inverse_difference(forecast, series);
    }
}