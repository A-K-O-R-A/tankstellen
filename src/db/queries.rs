
use super::connect::get_connection;

pub async fn save_station() -> Result<(), sqlx::Error> {
    let _ = sqlx::query(
        "
    INSERT INTO `tankstellen`.`stations` (
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
    ) ;
    ",
    )
    .execute(&mut get_connection().await)
    .await?;
    Ok(())
}
