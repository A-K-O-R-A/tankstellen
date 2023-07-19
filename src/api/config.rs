use std::env;

const API_URL: &'static str = "https://creativecommons.tankerkoenig.de";
const RADIUS: usize = 10;
const SORT: &'static str = "dist";
const FUEL_TYPE: &'static str = "all";

pub struct Config {
    pub api_url: String,
    pub radius: usize,
    pub sort: String,
    pub fuel_type: String,
    pub lat: String,
    pub lng: String,
    pub apikey: String,
}

pub fn get_config() -> Result<Config, env::VarError> {
    let lat = env::var("LAT")?;
    let lng = env::var("LNG")?;
    let apikey = env::var("APIKEY")?;

    Ok(Config {
        api_url: API_URL.into(),
        radius: RADIUS.into(),
        sort: SORT.into(),
        fuel_type: FUEL_TYPE.into(),
        lat,
        lng,
        apikey,
    })
}
