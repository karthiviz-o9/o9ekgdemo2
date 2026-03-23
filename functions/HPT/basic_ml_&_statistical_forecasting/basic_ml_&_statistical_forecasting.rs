use std::collections::VecDeque;

pub struct Demand {
    pub period: usize,
    pub value: f64,
}

pub struct Actuals {
    pub period: usize,
    pub value: f64,
}

pub struct Forecaster {
    smoothing_factor: f64,
}

impl Forecaster {
    pub fn new(smoothing_factor: f64) -> Self {
        Self { smoothing_factor }
    }

    pub fn forecast(&self, historical_demand: &[Demand], actuals: &[Actuals]) -> Vec<Demand> {
        let mut results = Vec::new();
        let mut smoothed_value = historical_demand.first().map(|d| d.value).unwrap_or(0.0);

        for (i, actual) in actuals.iter().enumerate() {
            let demand_val = historical_demand.get(i).map(|d| d.value).unwrap_or(smoothed_value);
            
            // Simple Linear Regression adjustment (slope approximation)
            let trend = if i > 0 {
                (demand_val - historical_demand[i - 1].value) * 0.1
            } else {
                0.0
            };

            // Exponential Smoothing
            smoothed_value = self.smoothing_factor * (actual.value + trend) 
                + (1.0 - self.smoothing_factor) * smoothed_value;

            results.push(Demand {
                period: actual.period,
                value: smoothed_value,
            });
        }

        results
    }
}

pub fn calculate_moving_average(data: &[Demand], window_size: usize) -> Vec<Demand> {
    let mut window = VecDeque::with_capacity(window_size);
    let mut results = Vec::new();

    for entry in data {
        window.push_back(entry.value);
        if window.len() > window_size {
            window.pop_front();
        }
        
        let avg = window.iter().sum::<f64>() / window.len() as f64;
        results.push(Demand {
            period: entry.period,
            value: avg,
        });
    }
    results
}