use api::requests::{get_close_stations, get_station_prices};
use std::error::Error;

use crate::db::connect::get_connection;

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
    // println!("{:#?}", stations);

    let station_ids: Vec<&str> = stations.iter().map(|s| s.id.as_str()).collect();

    let prices = get_station_prices(&station_ids).await?;
    println!("{:#?}", prices);

    Ok(())
}
