use super::connect::get_connection;

pub async fn create_tables() -> Result<(), sqlx::Error> {
    let _ = sqlx::query(
        "
    CREATE TABLE IF NOT EXISTS `tankstellen`.`stations` (
         `id` VARCHAR(36) NOT NULL ,
         `name` TEXT NOT NULL ,
         `brand` TEXT NOT NULL ,
         `street` TEXT NOT NULL ,
         `place` TEXT NOT NULL ,
         `lat` FLOAT NOT NULL ,
         `lng` FLOAT NOT NULL ,
         `dist` FLOAT NOT NULL ,
         `house_number` TEXT NOT NULL ,
         `post_code` INT NOT NULL ,
         UNIQUE `identifier` (`id`)
    ) ENGINE = InnoDB;
    ",
    )
    .execute(&mut get_connection().await)
    .await?;

    let _ = sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS `tankstellen`.`prices` (
            `id` VARCHAR(36) NOT NULL ,
            `diesel` FLOAT NOT NULL ,
            `e10` FLOAT NOT NULL ,
            `e5` FLOAT NOT NULL ,
            `timestamp` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            UNIQUE `indentifier` (`id`)
        ) ENGINE = InnoDB; 
    ",
    )
    .execute(&mut get_connection().await)
    .await?;

    Ok(())
}
