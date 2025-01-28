use std::fs;

use clap::Parser;
use gmap_api::GMap;
use types::Lead;

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

    /// Minimum rating threshold
    #[arg(short, long)]
    min_rating: Option<f32>,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenvy::dotenv()?;

    let args = Args::parse();

    let gmap_client = GMap::new()?;

    println!("Getting coordinates for {}...", args.city);
    let coordinates = gmap_client.get_coordinates(&args.city).await?;

    let radius = args.radius.clamp(1, 50) as f32;
    println!(
        "Getting places {}km around [{},{}]...",
        radius, coordinates.latitude, coordinates.longitude
    );

    let (leads, excluded) = gmap_client
        .get_places(
            &coordinates,
            radius,
            &args.business_types,
            &args.exclude_types,
            args.min_rating,
        )
        .await?;

    println!("\nFound {} potential leads:", leads.len());

    println!(
        "- Operational businesses: {}",
        leads.iter().filter(|l| l.is_operational).count()
    );

    if !excluded.is_empty() {
        println!("- Excluded businesses: {}", excluded.len());
    }

    if !leads.is_empty() {
        let result = export_leads(&leads, &args.city)?;
        println!("\n{}", result);
    } else {
        println!("\nNo leads to export");
    }

    // Export excluded leads
    if !excluded.is_empty() {
        let result = export_leads(&excluded, &format!("excluded_{}", args.city))?;
        println!("{}", result);
    }

    Ok(())
}

fn export_leads(leads: &[Lead], city: &str) -> Result<String, AppError> {
    const LEAD_DIR: &str = "leads";

    if leads.is_empty() {
        return Ok("No leads to export".to_string());
    }

    if !std::path::Path::new(LEAD_DIR).exists() {
        fs::create_dir(LEAD_DIR)?;
    }

    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let filename_base = format!("{}_{}", city, timestamp);

    let path = format!("{}/{}.json", LEAD_DIR, filename_base);
    let json = serde_json::to_string_pretty(leads)?;
    fs::write(&path, json)?;

    Ok(format!("Leads exported to {}", path))
}
