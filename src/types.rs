use serde::{Deserialize, Serialize};

//TODO: Only allow valid business types (https://developers.google.com/maps/documentation/places/web-service/place-types#table-a)
pub type BusinessType = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lead {
    pub name: Option<String>,
    pub address: Option<String>,
    pub google_maps_url: Option<String>,
    pub website: Option<String>,
    pub phone: Option<String>,
    pub rating: Option<f32>,
    pub total_ratings: Option<i32>,
    pub business_type: Vec<String>,
    pub business_status: Option<String>,
    pub is_operational: bool,
}
