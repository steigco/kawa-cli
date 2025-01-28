use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Google API key not found in environment variables")]
    MissingApiKey,

    #[error("No business types specified")]
    NoBusinessTypes,

    #[error("Error getting coordinates: {0}")]
    GeocodingError(String),

    #[error("API Error: {0}")]
    ApiError(String),

    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Environment Error: {0}")]
    EnvError(#[from] dotenvy::Error),

    #[error("Request Error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("JSON Error: {0}")]
    JsonError(#[from] serde_json::Error),
}
