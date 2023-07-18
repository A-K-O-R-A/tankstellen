use api::requests::{get_close_stations, get_station_prices};
use std::error::Error;

mod api;
mod db;

fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file.
    // Fails if .env file not found, not readable or invalid.
    dotenvy::dotenv()?;

    let stations = get_close_stations()?;
    // println!("{:#?}", stations);

    let station_ids: Vec<&str> = stations.iter().map(|s| s.id.as_str()).collect();

    let prices = get_station_prices(&station_ids)?;
    println!("{:#?}", prices);

    Ok(())
}
