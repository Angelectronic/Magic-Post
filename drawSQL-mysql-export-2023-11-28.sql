CREATE TABLE `package`(
    `id` CHAR(36) NOT NULL,
    `send_point` CHAR(36) NULL,
    `receive_point` CHAR(36) NULL,
    `cur_point` CHAR(36) NULL,
    `status` VARCHAR(255) NULL,
    `send_name` VARCHAR(255) NULL,
    `receive_name` VARCHAR(255) NULL,
    `send_date` DATE NULL,
    `required_date` DATE NULL,
    `shipped_date` DATE NULL,
    `send_ address` VARCHAR(255) NULL,
    `receive_address` VARCHAR(255) NULL,
    `send_phone` VARCHAR(255) NULL,
    `receive_phone` VARCHAR(255) NULL
);
ALTER TABLE
    `package` ADD PRIMARY KEY(`id`);
CREATE TABLE `package_delivery`(
    `delivery_id` CHAR(36) NOT NULL,
    `package_id` CHAR(36) NOT NULL
);
ALTER TABLE
    `package_delivery` ADD PRIMARY KEY(`delivery_id`);
CREATE TABLE `delivery`(
    `id` CHAR(36) NOT NULL,
    `start_point` CHAR(36) NULL,
    `end_point` CHAR(36) NULL,
    `begin_date` DATETIME NULL,
    `expected_date` DATETIME NULL,
    `arrived_date` DATETIME NULL
);
ALTER TABLE
    `delivery` ADD PRIMARY KEY(`id`);
CREATE TABLE `Employees`(
    `id` CHAR(36) NOT NULL,
    `name` VARCHAR(255) NULL,
    `position` VARCHAR(255) NULL,
    `point_id` CHAR(36) NULL,
    `username` VARCHAR(255) NOT NULL,
    `password` VARCHAR(255) NOT NULL
);
ALTER TABLE
    `Employees` ADD PRIMARY KEY(`id`);
CREATE TABLE `Points`(
    `id` CHAR(36) NOT NULL,
    `location` VARCHAR(255) NULL,
    `type` TINYINT(1) NULL COMMENT '0 là điểm giao dịch, 1 là điểm tập kết'
);
ALTER TABLE
    `Points` ADD PRIMARY KEY(`id`);
ALTER TABLE
    `delivery` ADD CONSTRAINT `delivery_end_point_foreign` FOREIGN KEY(`end_point`) REFERENCES `Points`(`id`);
ALTER TABLE
    `package` ADD CONSTRAINT `package_receive_point_foreign` FOREIGN KEY(`receive_point`) REFERENCES `Points`(`id`);
ALTER TABLE
    `package_delivery` ADD CONSTRAINT `package_delivery_package_id_foreign` FOREIGN KEY(`package_id`) REFERENCES `package`(`id`);
ALTER TABLE
    `package` ADD CONSTRAINT `package_cur_point_foreign` FOREIGN KEY(`cur_point`) REFERENCES `Points`(`id`);
ALTER TABLE
    `package_delivery` ADD CONSTRAINT `package_delivery_delivery_id_foreign` FOREIGN KEY(`delivery_id`) REFERENCES `delivery`(`id`);
ALTER TABLE
    `package` ADD CONSTRAINT `package_send_point_foreign` FOREIGN KEY(`send_point`) REFERENCES `Points`(`id`);
ALTER TABLE
    `Employees` ADD CONSTRAINT `employees_point_id_foreign` FOREIGN KEY(`point_id`) REFERENCES `Points`(`id`);
ALTER TABLE
    `delivery` ADD CONSTRAINT `delivery_start_point_foreign` FOREIGN KEY(`start_point`) REFERENCES `Points`(`id`);