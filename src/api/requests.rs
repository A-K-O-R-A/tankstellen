use crate::api::config::{get_config, Config};

pub fn get_close_stations() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_config()?;
    let url = format!("{}/json/list.php?lat={}&lng={}&rad={}&sort={}&type={}&apikey={}", config.);

    let resp = reqwest::blocking::get("https://httpbin.org/ip")?;
    println!("{:#?}", resp);
    Ok(())
}
