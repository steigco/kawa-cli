use clap::Parser;

mod error;
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

    println!("{:#?}", args);

    Ok(())
}
