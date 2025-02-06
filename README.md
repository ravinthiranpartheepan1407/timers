This package was actually built as a course module. In the course, I cover the math behind each time series model and build everything from scratch without using any external crates. Check out this course here: https://www.udemy.com/course/build-an-open-source-time-series-lib-from-scratch-in-rust

Library Modules:

[arima](https://docs.rs/timers-rs/0.1.1/timers_rs/arima/index.html)
[arma](https://docs.rs/timers-rs/0.1.1/timers_rs/arma/index.html)
[autocorrelation](https://docs.rs/timers-rs/0.1.1/timers_rs/autocorrelation/index.html)
[autoregressive](https://docs.rs/timers-rs/0.1.1/timers_rs/autoregressive/index.html)
[sarima](https://docs.rs/timers-rs/0.1.1/timers_rs/sarima/index.html)
[seasonality](https://docs.rs/timers-rs/0.1.1/timers_rs/seasonality/index.html)
[ses](https://docs.rs/timers-rs/0.1.1/timers_rs/ses/index.html)
[sma](https://docs.rs/timers-rs/0.1.1/timers_rs/sma/index.html)
[stationarity](https://docs.rs/timers-rs/0.1.1/timers_rs/stationarity/index.html)
[trend](https://docs.rs/timers-rs/0.1.1/timers_rs/trend/index.html)
[whitenoise](https://docs.rs/timers-rs/0.1.1/timers_rs/whitenoise/index.html)
[wma](https://docs.rs/timers-rs/0.1.1/timers_rs/wma/index.html)

Sample Usage:

```
use timers_rs::autocorrelation::autocorrelation;
use timers_rs::ses;

fn main() {

    let series = vec![10.0, 12.0, 14.0, 16.0, 18.0, 20.0];

    let model = ses::ExponentialSmoothing{
        alpha: 0.5,
        beta: Some(0.0),
    };

    let single_exponential_smoothing = model.single_exponential_smoothing(&series);
    println!("Single Exponential Smoothing: {:?}", single_exponential_smoothing);

    let autocorrelation_result = autocorrelation(&series, 2);
    println!("Autocorrealtion Result {}", autocorrelation_result);

}

```