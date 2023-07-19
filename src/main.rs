use api::requests;
use std::{error::Error, time::Duration};

use crate::db::{
    connect::get_connection,
    queries,
    types::{PriceEntry, StationEntry},
};
use tokio::{task, time};

mod api;
mod db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file.
    // Fails if .env file not found, not readable or invalid.
    dotenvy::dotenv()?;

    let mut conn = get_connection().await;
    sqlx::migrate!().run(&mut conn).await?;

    if let Ok(stations) = requests::get_close_stations().await {
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
            queries::save_station(entry).await?
        }
    }

    let station_ids: Vec<String> = queries::get_station_ids().await?;

    let forever = task::spawn(async move {
        // Run every 10 minutes
        let mut interval = time::interval(Duration::from_secs(60 * 10));

        loop {
            interval.tick().await;
            match do_price_update(&station_ids).await {
                Ok(_) => println!("Saved values"),
                Err(e) => println!("Error in loop: {e}"),
            }
        }
    });

    forever.await?;

    Ok(())
}

async fn do_price_update(close_ids: &[String]) -> Result<(), Box<dyn Error>> {
    let price_list = requests::get_station_prices(close_ids).await?;
    let price_entries: Vec<PriceEntry> = price_list
        .iter()
        .map(|p| PriceEntry {
            id: p.id.clone(),
            diesel: p.diesel,
            e10: p.e10,
            e5: p.e5,
        })
        .collect();
    queries::save_prices(&price_entries).await?;

    Ok(())
}
