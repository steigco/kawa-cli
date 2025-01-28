# Gelbe Seiten

A command-line tool to search for business leads using the Google Maps API. The tool allows you to search for businesses in a specific city within a given radius and export the results to JSON files.

## Features

* Search businesses by city and radius
* Filter by business types
* Exclude specific business types
* Set minimum rating threshold
* Export results to JSON files
* Separate tracking of excluded businesses

## Installation

1. Clone the repository
2. Install Rust if you haven't already (https://rustup.rs/)
3. Create a `.env` file in the root directory with your Google Maps API key:
    ```bash
    GOOGLE_MAPS_API_KEY=your_api_key_here
    ```

## Usage

```bash
cargo run -- -c <city> -r <radius> -t <business-type> -x <exclude-type> -m <min-rating>
```

### Arguments

| Argument | Description |
|----------|-------------|
| `-c, --city` | City to search for leads |
| `-r, --radius` | Search radius in kilometers (1-50) |
| `-t, --business-type` | Type of business to search for (can be used multiple times) |
| `-x, --exclude-type` | Type of business to exclude (can be used multiple times) |
| `-m, --min-rating` | Minimum rating threshold (0.0-5.0) |

## Example Commands
Search for restaurants in Munich with a minimum rating of 4.0:

```bash
cargo run -- -c Munich -r 5 -t restaurant -m 4.0
```

Search for multiple business types in Hamburg, excluding bars:

```bash
cargo run -- -c Hamburg -r 10 -t restaurant -t cafe -x bar
```

# Output

The tool creates JSON files in the `leads` directory.

Each lead contains the following information:

```json
{
    "place_id": "string",
    "name": "string",
    "address": "string",
    "rating": "float",
    "total_ratings": "integer",
    "types": ["string"],
    "location": {
        "lat": "float",
        "lng": "float"
    }
}
```

## Notes

- The Google Maps API has usage limits and may incur costs
- Business types should match Google's place types documentation
- Maximum results per request are limited to 20 places
- Phone numbers are currently not fetched (requires call to pricier SKU): I will implement this using a flag in the future