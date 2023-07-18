use sqlx::{Connection, MySqlConnection};
use std::{env, error::Error};

pub async fn connect() -> Result<MySqlConnection, Box<dyn Error>> {
    let db_url = env::var("DATABASE_URL")?;
    let conn = MySqlConnection::connect(db_url.as_str()).await?;

    Ok(conn)
}
