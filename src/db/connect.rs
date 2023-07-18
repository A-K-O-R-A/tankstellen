use sqlx::{Connection, MySqlConnection};
use std::env;

pub async fn get_connection() -> MySqlConnection {
    let db_url = env::var("DATABASE_URL").expect("No DATABASE_URL in env");
    let conn = MySqlConnection::connect(db_url.as_str()).await;

    match conn {
        Ok(conn) => conn,
        Err(e) => panic!("Unable to connect to database: {e}"),
    }
}
