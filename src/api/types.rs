use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ListResponse {
    pub ok: bool,
    pub license: String,
    pub data: String,
    pub status: String,
    pub stations: Vec<Station>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Station {
    pub id: String,
    pub name: String,
    pub brand: String,
    pub street: String,
    pub place: String,
    pub lat: f32,
    pub lng: f32,
    pub dist: f32,
    pub diesel: f32,
    pub e5: f32,
    pub e10: f32,
    #[serde(rename = "isOpen")]
    pub is_open: bool,
    #[serde(rename = "houseNumber")]
    pub house_number: String,
    #[serde(rename = "postCode")]
    pub post_code: usize,
}

#[derive(Serialize, Deserialize)]
pub struct PricesResponse {
    pub ok: bool,
    pub license: String,
    pub data: String,
    pub prices: HashMap<String, StationPriceInfo>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum StationPriceInfo {
    #[serde(rename = "open")]
    Open { e5: f32, e10: f32, diesel: f32 },
    #[serde(rename = "closed")]
    Closed {},
    #[serde(rename = "no prices")]
    NoPrices {},
}

#[derive(Debug)]
pub struct StationPrice {
    pub id: String,
    pub e5: f32,
    pub e10: f32,
    pub diesel: f32,
}
