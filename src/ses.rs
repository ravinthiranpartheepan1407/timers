use crate::trend;

pub struct ExponentialSmoothing{
    pub alpha: f64,
    pub beta: Option<f64>,
}

impl ExponentialSmoothing{
    pub fn single_exponential_smoothing(&self, data: &[f64]) -> Vec<f64>{
        let alpha = self.alpha;
        let mut smoothed = Vec::new();
        let mut x = data[0];

        for &s in data{
            x = alpha * s + (1.0 - alpha) * s;
            println!("X Calculation: {}", x);
            smoothed.push(x);
        }

        return smoothed
    }

    pub fn holt_linear_model(&self, data: &[f64]) -> (Vec<f64>, Vec<f64>){
        let alpha = self.alpha;
        let beta = self.beta.expect("Beta param is required for Holt Winter Model");

        let mut smoothed = Vec::new();
        let mut trend_smoothed = Vec::new();
        let mut s = data[0];
        let mut t = data[1] - data[0];

        for &x in data{
            let prev_s = s;
            s = alpha * x + (1.0 - alpha) * (s + t); //Level estimation equation
            t = beta * (s - prev_s) + (1.0 - beta) * t; //Trend Estimation Equation
            smoothed.push(s);
            trend_smoothed.push(t);
        }

        return (smoothed, trend_smoothed)
    }
}