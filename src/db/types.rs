#[derive(sqlx::FromRow)]
pub struct PriceEntry {
    pub id: String,
    pub diesel: f32,
    pub e10: f32,
    pub e5: f32,
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
