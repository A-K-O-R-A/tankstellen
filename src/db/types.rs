#[derive(sqlx::FromRow)]
pub struct PriceEntry {
    pub id: String,
    pub diesel: u64,
    pub e10: u64,
    pub e5: u64,
}

#[derive(sqlx::FromRow)]
pub struct StationEntry {
    pub id: String,
    pub name: String,
    pub brand: String,
    pub street: String,
    pub place: String,
    pub lat: f32,
    pub lng: f32,
    pub dist: f32,
    pub house_number: String,
    pub post_code: i32,
}
