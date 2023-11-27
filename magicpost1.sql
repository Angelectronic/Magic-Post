CREATE TABLE `customer`(
    `id` CHAR(36) NOT NULL,
    `username` VARCHAR(255) NOT NULL,
    `password` VARCHAR(255) NOT NULL
);
ALTER TABLE
    `customer` ADD PRIMARY KEY(`id`);
CREATE TABLE `package`(
    `id` CHAR(36) NOT NULL,
    `status` VARCHAR(255) NOT NULL,
    `sender_id` CHAR(36) NOT NULL,
    `sent_date` DATE NOT NULL,
    `required_date` DATE NOT NULL,
    `shipped_date` DATE NOT NULL,
    `sent_point` CHAR(36) NOT NULL,
    `received_point` CHAR(36) NOT NULL
);
ALTER TABLE
    `package` ADD PRIMARY KEY(`id`);
CREATE TABLE `Employees`(
    `id` CHAR(36) NOT NULL,
    `name` VARCHAR(255) NOT NULL,
    `dob` DATE NOT NULL,
    `position` VARCHAR(255) NOT NULL,
    `point_id` BIGINT NOT NULL
);
ALTER TABLE
    `Employees` ADD PRIMARY KEY(`id`);
CREATE TABLE `Points`(
    `id` CHAR(36) NOT NULL,
    `location` VARCHAR(255) NOT NULL,
    `leader_id` CHAR(36) NOT NULL,
    `type` TINYINT(1) NOT NULL COMMENT '0 là điểm giao dịch, 1 là điểm tập kết'
);
ALTER TABLE
    `Points` ADD PRIMARY KEY(`id`);
ALTER TABLE
    `package` ADD CONSTRAINT `package_received_point_foreign` FOREIGN KEY(`received_point`) REFERENCES `Points`(`id`);
ALTER TABLE
    `package` ADD CONSTRAINT `package_sender_id_foreign` FOREIGN KEY(`sender_id`) REFERENCES `customer`(`id`);
ALTER TABLE
    `package` ADD CONSTRAINT `package_sent_point_foreign` FOREIGN KEY(`sent_point`) REFERENCES `Points`(`id`);
ALTER TABLE
    `Points` ADD CONSTRAINT `points_leader_id_foreign` FOREIGN KEY(`leader_id`) REFERENCES `Employees`(`id`);