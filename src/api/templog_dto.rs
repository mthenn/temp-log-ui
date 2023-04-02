use chrono::{DateTime, Utc};

pub struct Measurement {
    temperature: f64,
    humidity: f64,
    timestamp: DateTime<Utc>,
}
