use sqlx::{mysql::MySqlRow, Row};

use super::{
    connect::get_connection,
    types::{PriceEntry, StationEntry},
};

pub async fn save_station(station: StationEntry) -> Result<(), sqlx::Error> {
    let mut conn = get_connection().await;

    let row = sqlx::query(
        "
    SELECT `id` FROM `stations`
    WHERE `id`= ?;
    ",
    )
    .bind(&station.id)
    .fetch_optional(&mut conn)
    .await?;

    if row.is_some() {
        return Ok(());
    }

    let _ = sqlx::query(
        "
    INSERT INTO `stations` (
         `id`,
         `name`,
         `brand`,
         `street`,
         `place`,
         `lat`,
         `lng`,
         `dist`,
         `house_number`,
         `post_code`
    ) VALUES ( ? , ? , ? , ? , ? , ? , ? , ? , ? , ? );
    ",
    )
    .bind(station.id)
    .bind(station.name)
    .bind(station.brand)
    .bind(station.street)
    .bind(station.place)
    .bind(station.lat)
    .bind(station.lng)
    .bind(station.dist)
    .bind(station.house_number)
    .bind(station.post_code)
    .execute(&mut conn)
    .await?;

    Ok(())
}

pub async fn save_prices(price_list: &[PriceEntry]) -> Result<(), sqlx::Error> {
    let mut conn = get_connection().await;
    let entry_count = price_list.len();

    if price_list.is_empty() {
        return Ok(());
    }

    let values_placeholder = vec!["(?,?,?,?)"; entry_count].join(",");
    let sql = format!(
        "
    INSERT INTO `prices` (
        `id`,
        `diesel`,
        `e10`,
        `e5`
    ) VALUES {};
    ",
        values_placeholder
    );

    let mut query = sqlx::query(&sql);
    for entry in price_list {
        query = query
            .bind(entry.id.as_str())
            .bind(entry.diesel)
            .bind(entry.e10)
            .bind(entry.e5)
    }

    let row_count = query.execute(&mut conn).await?.rows_affected();

    if row_count != entry_count as u64 {
        println!(
            "Tried inserting {} rows but only inserted {} rows",
            entry_count, row_count
        )
    }

    Ok(())
}

pub async fn get_station_ids() -> Result<Vec<String>, sqlx::Error> {
    sqlx::query(
        "
        SELECT `id` FROM `stations`
        ORDER BY `dist` ASC
        LIMIT 10;
        ",
    )
    .map(|r: MySqlRow| r.get("id"))
    .fetch_all(&mut get_connection().await)
    .await
}
