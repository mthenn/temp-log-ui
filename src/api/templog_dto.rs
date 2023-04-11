use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Measurement {
    temperature: f64,
    humidity: f64,
    timestamp: DateTime<Utc>,
}
