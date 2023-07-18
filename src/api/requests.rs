use super::config::get_config;
use super::types::{ListResponse, PricesResponse, Station, StationPrice, StationPriceInfo};
use std::error::Error;

pub fn get_close_stations() -> Result<Vec<Station>, Box<dyn Error>> {
    let config = get_config()?;
    let request_url = format!(
        "{}/json/list.php?lat={}&lng={}&rad={}&sort={}&type={}&apikey={}",
        config.api_url,
        config.lat,
        config.lng,
        config.radius,
        config.sort,
        config.fuel_type,
        config.apikey
    );

    let resp = reqwest::blocking::get(request_url)?;
    let content = resp.text()?;

    // Parse the string of data
    let resp: ListResponse = serde_json::from_str(&content)?;

    Ok(resp.stations)
}

pub fn get_station_prices(ids: &[&str]) -> Result<Vec<StationPrice>, Box<dyn Error>> {
    let config = get_config()?;

    let ids_string = ids.join(",");
    let request_url = format!(
        "{}/json/prices.php?ids={}&apikey={}",
        config.api_url, ids_string, config.apikey
    );

    let resp = reqwest::blocking::get(request_url)?;
    let content = resp.text()?;

    // Parse the string of data
    let resp: PricesResponse = serde_json::from_str(&content)?;

    let prices: Vec<_> = resp
        .prices
        .into_iter()
        .filter(|(_, v)| match v {
            StationPriceInfo::Open { .. } => true,
            _ => false,
        })
        .map(|(k, v)| match v {
            StationPriceInfo::Open { diesel, e10, e5 } => StationPrice {
                id: k,
                diesel,
                e10,
                e5,
            },
            _ => unreachable!("Every other case is covered by the filter"),
        })
        .collect();

    Ok(prices)
}
