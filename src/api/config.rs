use std::env;

const API_URL: &'static str = "https://creativecommons.tankerkoenig.de";
const RADIUS: usize = 4;
const SORT: &'static str = "dist";
const FUEL_TYPE: &'static str = "all";

pub struct Config {
    api_url: String,
    radius: usize,
    sort: String,
    fuel_type: String,
    lat: String,
    lng: String,
    apikey: String,
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
