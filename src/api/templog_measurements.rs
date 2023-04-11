use chrono::{DateTime, Utc};

use super::templog_dto::Measurement;

pub async fn get_measurements(
    begin_timestamp: DateTime<Utc>,
    end_timestamp: DateTime<Utc>,
) -> Result<Vec<Measurement>, ()> {
    let client = reqwest::Client::builder()
        .build()
        .expect("Got error during REST client creation.");

    let response = client
        .get("templog/measurements")
        .query(&("date_from", begin_timestamp))
        .query(&("date_to", end_timestamp))
        .send()
        .await
        .expect("Get measurements failed.");

    if !response.status().is_success() {
        panic!(
            "The request was not successful! Status code: {}",
            response.status()
        );
    }
    println!("Sent reading to backend.");
}
