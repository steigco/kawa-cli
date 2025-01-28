use reqwest::Client;
use serde_json::Value;

use crate::{error::AppError, types::Coordinates};

pub struct GMap {
    client: Client,
    api_key: String,
}

impl GMap {
    pub fn new() -> Result<Self, AppError> {
        let api_key = std::env::var("GOOGLE_API_KEY").map_err(|_| AppError::MissingApiKey)?;

        Ok(Self {
            client: Client::new(),
            api_key,
        })
    }

    pub async fn get_coordinates(&self, city: &str) -> Result<Coordinates, AppError> {
        let url = format!(
            "https://maps.googleapis.com/maps/api/geocode/json?address={}&key={}",
            city, self.api_key
        );

        let response = self.client.get(url).send().await?.json::<Value>().await?;

        if let Some(location) = response
            .get("results")
            .and_then(|r| r.get(0))
            .and_then(|r| r.get("geometry"))
            .and_then(|g| g.get("location"))
        {
            Ok(Coordinates {
                latitude: location["lat"].as_f64().unwrap_or_default(),
                longitude: location["lng"].as_f64().unwrap_or_default(),
            })
        } else {
            Err(AppError::GeocodingError(
                "Could not find coordinates".into(),
            ))
        }
    }
}
