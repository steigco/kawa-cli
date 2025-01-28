use reqwest::Client;
use serde_json::Value;

use crate::{
    error::AppError,
    types::{BusinessType, Coordinates, Lead},
};

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

    pub async fn get_places(
        &self,
        coordinates: &Coordinates,
        radius: f32,
        included_types: &[BusinessType],
        excluded_types: &[BusinessType],
        min_rating: Option<f32>,
    ) -> Result<(Vec<Lead>, Vec<Lead>), AppError> {
        let mut all_leads = Vec::new();
        let mut excluded_leads = Vec::new();

        let request_body = serde_json::json!({
            "includedTypes": included_types,
            "locationRestriction": {
                "circle": {
                    "center": {
                        "latitude": coordinates.latitude,
                        "longitude": coordinates.longitude
                    },
                    "radius": radius * 1000.0
                }
            },
            "maxResultCount": 20 // Must be between 10 and 20
        });

        let response = self.client
                    .post("https://places.googleapis.com/v1/places:searchNearby")
                    .header("Content-Type", "application/json")
                    .header("X-Goog-Api-Key", &self.api_key)
                    .header("X-Goog-FieldMask", "places.id,places.displayName,places.formattedAddress,places.googleMapsUri,places.websiteUri,places.primaryType,places.types,places.rating,places.userRatingCount,places.businessStatus")
                    .json(&request_body)
                    .send()
                    .await?;

        if response.status() != 200 {
            return Err(AppError::ApiError(format!(
                "Places API error: {}",
                response.status()
            )));
        }

        let response = response.json::<Value>().await?;

        let empty_vec = Vec::new();
        let places = response["places"].as_array().unwrap_or(&empty_vec);

        for place in places {
            let place_types: Vec<String> = place["types"]
                .as_array()
                .map(|types| {
                    types
                        .iter()
                        .filter_map(|t| t.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default();

            let lead = Lead {
                name: place["displayName"]["text"].as_str().map(String::from),
                address: place["formattedAddress"].as_str().map(String::from),
                google_maps_url: place["googleMapsUri"].as_str().map(String::from),
                website: place["websiteUri"].as_str().map(String::from),
                phone: None, // Phone number requires a separate details request
                rating: place["rating"].as_f64().map(|r| r as f32),
                total_ratings: place["userRatingCount"].as_i64().map(|r| r as i32),
                business_type: place_types.clone(),
                business_status: place["businessStatus"].as_str().map(String::from),
                is_operational: place["businessStatus"].as_str() == Some("OPERATIONAL"),
            };

            if let Some(min_r) = min_rating {
                if lead.rating.unwrap_or(0.0) < min_r {
                    continue;
                }
            }

            if excluded_types.iter().any(|t| place_types.contains(t)) {
                excluded_leads.push(lead);
            } else {
                all_leads.push(lead);
            }
        }

        Ok((all_leads, excluded_leads))
    }
}
