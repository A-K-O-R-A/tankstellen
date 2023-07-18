use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ListResponse {
    ok: bool,
    license: String,
    data: String,
    status: String,
    stations: Vec<Station>,
}

#[derive(Serialize, Deserialize)]
struct Station {
    id: String,
    name: String,
    brand: String,
    street: String,
    place: String,
    lat: f32,
    lng: f32,
    dist: f32,
    diesel: f32,
    e5: f32,
    e10: f32,
    #[serde(rename = "isOpen")]
    is_open: bool,
    #[serde(rename = "houseNumber")]
    house_number: String,
    #[serde(rename = "postCode")]
    post_code: usize,
}

#[derive(Serialize, Deserialize)]
struct PriceResponse {
    ok: bool,
    license: String,
    data: String,
    status: String,
    prices: HashMap<String, StationPriceInfo>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "status")]
enum StationPriceInfo {
    #[serde(rename = "open")]
    Open { e5: f32, e10: f32, diesel: f32 },
    #[serde(rename = "closed")]
    Closed {},
    #[serde(rename = "no prices")]
    NoPrices {},
}
