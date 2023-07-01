use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Measurement {
    pub temperature: f64,
    pub humidity: f64,
    pub timestamp: DateTime<Utc>,
}
