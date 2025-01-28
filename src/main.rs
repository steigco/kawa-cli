use clap::Parser;
use gmap_api::GMap;

mod error;
mod gmap_api;
mod types;

use crate::error::AppError;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// City to search for leads
    #[arg(short, long)]
    city: String,

    /// Search radius in kilometers
    #[arg(short, long)]
    radius: u32,

    /// Type of business to search for (comma-separated or multiple flags)
    #[arg(short = 't', long = "business-type")]
    business_types: Vec<String>,

    /// Type of business to exclude (comma-separated or multiple flags)
    #[arg(short = 'x', long = "exclude-type")]
    exclude_types: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenvy::dotenv()?;

    let args = Args::parse();

    let gmap_client = GMap::new()?;

    println!("Getting coordinates for {}...", args.city);
    let coordinates = gmap_client.get_coordinates(&args.city).await?;

    let radius = args.radius.clamp(1, 50);
    println!(
        "Getting leads {}km around [{},{}]...",
        radius, coordinates.latitude, coordinates.longitude
    );

    Ok(())
}
