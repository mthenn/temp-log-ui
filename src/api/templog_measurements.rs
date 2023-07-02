use chrono::{DateTime, Utc};
use reqwest::Error;

use super::templog_dto::Measurement;

pub async fn get_measurements(
    begin_timestamp: DateTime<Utc>,
    end_timestamp: DateTime<Utc>,
) -> Result<Vec<Measurement>, Error> {
    let client = reqwest::Client::builder().build()?;

    // Read backend url from environment variable at compile time
    let base_url = match option_env!("BACKEND_BASE_URL") {
        Some(value) => value.to_string(),
        None => {
            log::warn!("No backend url has been set, application will not work.");
            "".to_string()
        }
    };

    let response = client
        .get(base_url + "/api/measurements")
        .query(&[("date_from", begin_timestamp), ("date_to", end_timestamp)])
        .send()
        .await?
        .json::<Vec<Measurement>>()
        .await?;

    Ok(response)
}
