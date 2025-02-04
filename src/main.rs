// use core::time;
// use std::result;

// use timers::autocorrelation;
// use timers::autocorrelation::autocorrelation;
// use timers::stationarity;
// use timers::trend;
// use timers::seasonality;
// use timers::whitenoise;
// use timers::autoregressive;
// use timers::sma;
// use timers::wma;
// use timers::arma;
// use timers::arima;
// use timers::sarima;
// use timers::ses;

// fn main(){
//     // let data = vec![1.0,2.0,3.0,4.0,5.0];
//     // let data = vec![1.70, 2.0, 2.3, 2.6];
//     // let window_size = 3;
//     // let lag = 7;

//     // let result = autocorrelation::autocorrelation(&data, lag);
//     // println!("Autocorrelation at lag {}: {}", lag, result);

//     // let window = 4;
//     // let result = stationarity::check_stationarity(&data, window);
//     // println!("Is it stationarity? for the given time series using given window size: {} -> {}", window, result);

//     // let result = trend::detect_trend(&data);
//     // println!("Check for trend: {}", result);

//     // let period = 7;

//     // let result = seasonality::check_seasonality(&data, period);
//     // if result{
//     //     println!("Seasonality Detected in the time series")
//     // } else{
//     //     println!("No Seasonality Detected in the time series")
//     // }
//     // let lag = 1;
//     // let result = whitenoise::check_white_noise(&data, lag);
//     // if result{
//     //     println!("The time series is white noise");
//     // }else{
//     //     println!("The time series is not white noise");
//     // }

//     // let coeffresult = autoregressive::fit_ar_model(&data, lag);
//     // println!("Lag({}) and Computed Coefficients: {:?}", lag, coeffresult);
//     // let result = autoregressive::predict_ar_model(&data, lag, &coeffresult);
//     // println!("Predictions: {:?}", result);
//     // let result = sma::simple_moving_average(&data, window_size);
//     // println!("Moving Average with window size: {} values: {:?}", window_size, result)

//     // let weights = vec![1.0,2.0,3.0];
//     // let result = wma::weighted_moving_average(&data, window_size, &weights);
//     // println!("Window Size: {} | Weighted Mean Average: {:?}", window_size, result)

//     // ARMA in order of (2 -> AR, 1 -> MA)
//     // let ar_coeffs = vec![0.7, -0.2];
//     // let ma_coeffs = vec![0.5];
//     // let c = 1.0;
//     // let n = 100;

//     // let result = arma::arma(&ar_coeffs, &ma_coeffs, c, n);
//     // println!("{:?}", &result[..10]);

//     let series = vec![10.7, 12.2, 13.1, 16.2, 18.4, 20.1];
//     // let mut arima_instance = arima::ARIMA::new(1,1,1, vec![0.8], vec![0.2]);
//     // let differenced_series = arima_instance.difference(&series);
//     // println!("Differenced Series: {:?}", differenced_series);

//     // let prediction = arima_instance.predict(&series);
//     // println!("Next Prediction: {}", prediction);

//     // Create SARIMA model with some arbitrary coefficients
//     // let mut sarima_instance = sarima::SARIMA::new(
//     //     1,1,1,1,1,1, 4,vec![0.8], vec![0.2], vec![0.5], vec![0.3],
//     // );

//     // let differenced_series = sarima_instance.difference(&series);
    
//     // println!("Differenced Series: {:?}", differenced_series);

//     // // Predict next value
//     // let prediction = sarima_instance.predict(&series);
//     // println!("Next Prediction: {}", prediction);

//     let model = ses::ExponentialSmoothing{
//         alpha: 0.3,
//         beta: Some(0.3),
//     };

//     // let result = model.single_exponential_smoothing(&series);
//     // println!("Single Exponential Smoothing: {:?}", result);

//     let result = model.holt_linear_model(&series);
//     println!("Holt Linear Model - Level: {:?}", result.0);
//     println!("Holt Linear Model - Trend: {:?}", result.1);


// }