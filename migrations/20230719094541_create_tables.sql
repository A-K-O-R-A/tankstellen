CREATE TABLE IF NOT EXISTS `stations` (
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
);

CREATE TABLE IF NOT EXISTS `prices` (
    `id` VARCHAR(36) NOT NULL ,
    `diesel` FLOAT NOT NULL ,
    `e10` FLOAT NOT NULL ,
    `e5` FLOAT NOT NULL ,
    `timestamp` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
); 

ALTER TABLE `prices` ADD INDEX ( `timestamp` );