use api::requests::{get_close_stations, get_station_prices};
use std::error::Error;

use crate::db::{
    connect::get_connection,
    queries::{save_prices, save_station},
    types::{PriceEntry, StationEntry},
};

mod api;
mod db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file.
    // Fails if .env file not found, not readable or invalid.
    dotenvy::dotenv()?;

    let mut conn = get_connection().await;
    sqlx::migrate!().run(&mut conn).await?;

    let stations = get_close_stations().await?;
    let station_ids: Vec<String> = stations.iter().map(|s| s.id.clone()).collect();

    for station in stations {
        let entry = StationEntry {
            id: station.id,
            name: station.name,
            brand: station.brand,
            street: station.street,
            place: station.place,
            lat: station.lat,
            lng: station.lng,
            dist: station.dist,
            house_number: station.house_number,
            post_code: station.post_code as i32,
        };
        save_station(entry).await?
    }

    let close_ids = &station_ids[..10];

    let price_list = get_station_prices(close_ids).await?;
    let price_entries: Vec<PriceEntry> = price_list
        .iter()
        .map(|p| PriceEntry {
            id: p.id.clone(),
            diesel: p.diesel,
            e10: p.e10,
            e5: p.e5,
        })
        .collect();
    save_prices(&price_entries).await?;

    Ok(())
}
