use chrono::{DateTime, Utc};
use reqwest::Error;

use super::templog_dto::Measurement;

pub async fn get_measurements(
    begin_timestamp: DateTime<Utc>,
    end_timestamp: DateTime<Utc>,
) -> Result<Vec<Measurement>, Error> {
    let client = reqwest::Client::builder().build()?;

    let response = client
        .get("templog/measurements")
        .query(&("date_from", begin_timestamp))
        .query(&("date_to", end_timestamp))
        .send()
        .await?
        .json::<Vec<Measurement>>()
        .await?;

    Ok(response)
}
