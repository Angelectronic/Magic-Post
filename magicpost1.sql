-- phpMyAdmin SQL Dump
-- version 5.2.1
-- https://www.phpmyadmin.net/
--
-- Máy chủ: 127.0.0.1
-- Thời gian đã tạo: Th12 28, 2023 lúc 11:35 AM
-- Phiên bản máy phục vụ: 10.4.32-MariaDB
-- Phiên bản PHP: 8.0.30

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
START TRANSACTION;
SET time_zone = "+00:00";


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;

--
-- Cơ sở dữ liệu: `magic_post`
--

-- --------------------------------------------------------

--
-- Cấu trúc bảng cho bảng `cur_point_history`
--

CREATE TABLE `cur_point_history` (
  `point_id` char(36) NOT NULL,
  `package_id` char(36) NOT NULL,
  `time` datetime NOT NULL,
  `status` varchar(255) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Đang đổ dữ liệu cho bảng `cur_point_history`
--

INSERT INTO `cur_point_history` (`point_id`, `package_id`, `time`, `status`) VALUES
('084bd1d1-8945-11ee-b789-b05cdad83c7f', 'bd5bee42-894b-11ee-b789-b05cdad83c7f', '2023-11-12 16:56:52', 'send');

-- --------------------------------------------------------

--
-- Cấu trúc bảng cho bảng `delivery`
--

CREATE TABLE `delivery` (
  `id` char(36) NOT NULL,
  `start_point` char(36) NOT NULL,
  `end_point` char(36) NOT NULL,
  `begin_date` datetime NOT NULL,
  `arrived_date` datetime DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Đang đổ dữ liệu cho bảng `delivery`
--

INSERT INTO `delivery` (`id`, `start_point`, `end_point`, `begin_date`, `arrived_date`) VALUES
('cafcbaa5-a563-11ee-8a45-b05cdad83c7f', '084bd1d1-8945-11ee-b789-b05cdad83c7f', '084bdf03-8945-11ee-b789-b05cdad83c7f', '2023-12-28 10:30:25', NULL);

-- --------------------------------------------------------

--
-- Cấu trúc bảng cho bảng `employees`
--

CREATE TABLE `employees` (
  `id` char(36) NOT NULL,
  `reference` varchar(255) NOT NULL,
  `create_date` date DEFAULT NULL,
  `last_seen` datetime DEFAULT NULL,
  `name` varchar(255) DEFAULT NULL,
  `sex` varchar(36) NOT NULL DEFAULT 'male',
  `email` varchar(255) DEFAULT NULL,
  `birthday` date DEFAULT NULL,
  `phone` varchar(255) DEFAULT NULL,
  `position` varchar(255) DEFAULT NULL,
  `point_id` char(36) DEFAULT NULL,
  `username` varchar(255) NOT NULL,
  `password` varchar(255) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Đang đổ dữ liệu cho bảng `employees`
--

INSERT INTO `employees` (`id`, `reference`, `create_date`, `last_seen`, `name`, `sex`, `email`, `birthday`, `phone`, `position`, `point_id`, `username`, `password`) VALUES
('1102e612-8ecf-11ee-8c8f-b05cdad83c7f', 'ABC', '2023-12-08', '2023-12-28 16:10:32', 'Phạm Văn Hùng', 'male', NULL, NULL, '', 'leader', '0215a62a-8944-11ee-b789-b05cdad83c7f', 'leader1', '$2b$12$LfPiJzMJQxpJWnba8WreseM8p9h6XudjSZ.emmJB8n7IPvGFkg1b6'),
('285f6ae9-956c-11ee-8651-b05cdad83c7f', 'BDC', '2023-12-08', NULL, 'hieu', 'male', NULL, NULL, '', 'subordinate', '0215a62a-8944-11ee-b789-b05cdad83c7f', 'hieu2', '$2b$12$VWRvsZlb6U1qbUDEgZlXPeDXDX5.Zxw18Nw38I6rgM6xbFrGhmaxS'),
('36cbbf10-8ecf-11ee-8c8f-b05cdad83c7f', 'UDV', '2023-12-08', NULL, 'Huỳnh Thế Công', 'male', NULL, NULL, '', 'leader', '021599a1-8944-11ee-b789-b05cdad83c7f', 'leader2', '$2b$12$/fo3UJI/srr6W5f3vNjGxeoPsUlnuWMlWLVhBkdAFXwNh67geUh4y'),
('485cf9b3-8ecf-11ee-8c8f-b05cdad83c7f', 'DEF', '2023-12-08', NULL, 'Đỗ Mạnh Hoa', 'male', NULL, NULL, '', 'leader', '084bd1d1-8945-11ee-b789-b05cdad83c7f', 'leader3', '$2b$12$IYzrthDGj8lQ35AWfusYtuYoD.bmVBiLANJjP6oKbYNtKFdgWF5Gy'),
('594a6490-8ecf-11ee-8c8f-b05cdad83c7f', 'XYZ', '2023-12-08', NULL, 'Thế Quang', 'male', NULL, NULL, '', 'leader', '084bdf03-8945-11ee-b789-b05cdad83c7f', 'leader4', '$2b$12$4dxKofFmHXaZhsSwLkKIC.u8GogUC2lhDv0Z6WNW0E1RQFwtQV0ji'),
('69c0a3df-8ecf-11ee-8c8f-b05cdad83c7f', 'POI', '2023-12-08', NULL, 'Tuấn Khôi', 'male', NULL, NULL, '', 'leader', '1c54efa7-8945-11ee-b789-b05cdad83c7f', 'leader5', '$2b$12$2ewviyB8lCE230QY5e9t8.wagyx/cVJTqqZ9HfeBTI7R.r3nYiXJi'),
('9f2d9992-8ece-11ee-8c8f-b05cdad83c7f', 'ELO', '2023-12-08', '2023-12-28 16:10:28', 'Elon Musk', 'male', NULL, NULL, '', 'CEO', NULL, 'ceo', '$2b$12$/ydOvHRYN/3m3Cz4xy7XO.EhEsbWF/WfYMi6S8JFCNt1V6Q4mfecW');

-- --------------------------------------------------------

--
-- Cấu trúc bảng cho bảng `items`
--

CREATE TABLE `items` (
  `item_name` varchar(255) NOT NULL DEFAULT '',
  `quantity` int(11) DEFAULT NULL,
  `value` int(11) NOT NULL DEFAULT 0,
  `package_id` char(36) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Đang đổ dữ liệu cho bảng `items`
--

INSERT INTO `items` (`item_name`, `quantity`, `value`, `package_id`) VALUES
('Đồng hồ', 1, 100000, '0144fd37-957b-11ee-90fa-b05cdad83c7f'),
('Điện thoại', 1, 10000000, '0144fd37-957b-11ee-90fa-b05cdad83c7f'),
('Tài liệu', NULL, 100000, 'bd5bee42-894b-11ee-b789-b05cdad83c7f');

-- --------------------------------------------------------

--
-- Cấu trúc bảng cho bảng `package`
--

CREATE TABLE `package` (
  `id` char(36) NOT NULL,
  `package_id` varchar(255) NOT NULL,
  `send_name` varchar(255) DEFAULT NULL,
  `send_date` date DEFAULT NULL,
  `send_phone` varchar(255) DEFAULT NULL,
  `send_address` varchar(255) DEFAULT NULL,
  `send_point` char(36) DEFAULT NULL,
  `receive_name` varchar(255) DEFAULT NULL,
  `receive_phone` varchar(255) DEFAULT NULL,
  `receive_address` varchar(255) DEFAULT NULL,
  `receive_point` char(36) DEFAULT NULL,
  `cur_point` char(36) DEFAULT NULL,
  `next_point` char(36) DEFAULT NULL,
  `status` varchar(255) DEFAULT NULL,
  `main_cost` int(11) NOT NULL DEFAULT 0,
  `other_cost` int(11) NOT NULL DEFAULT 0,
  `gtgt_cost` int(11) NOT NULL DEFAULT 0,
  `other_service_cost` int(11) NOT NULL DEFAULT 0,
  `total_cost` int(11) NOT NULL DEFAULT 0,
  `vat` int(11) NOT NULL DEFAULT 0,
  `package_type` tinyint(4) NOT NULL DEFAULT 0,
  `instruction_type` tinyint(4) DEFAULT NULL,
  `weight` float NOT NULL DEFAULT 0,
  `special_service` varchar(255) NOT NULL DEFAULT '',
  `note` varchar(255) NOT NULL DEFAULT '',
  `cod` int(11) NOT NULL DEFAULT 0,
  `receive_other_cost` int(11) NOT NULL DEFAULT 0
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Đang đổ dữ liệu cho bảng `package`
--

INSERT INTO `package` (`id`, `package_id`, `send_name`, `send_date`, `send_phone`, `send_address`, `send_point`, `receive_name`, `receive_phone`, `receive_address`, `receive_point`, `cur_point`, `next_point`, `status`, `main_cost`, `other_cost`, `gtgt_cost`, `other_service_cost`, `total_cost`, `vat`, `package_type`, `instruction_type`, `weight`, `special_service`, `note`, `cod`, `receive_other_cost`) VALUES
('0144fd37-957b-11ee-90fa-b05cdad83c7f', 'VN1235HN', 'Tâm', '2023-11-01', '0937128323', 'Tầng 1, Tòa nhà Báo Lao Động, 06 P. Phạm Văn Bạch, Yên Hoà, Cầu Giấy, Hà Nội', '0215a62a-8944-11ee-b789-b05cdad83c7f', 'Nam', '0314912333', '73-75 Thủ Khoa Huân, Phường Bến Thành, Quận 1, Thành phố Hồ Chí Minh 70000, Việt Nam', '1c54efa7-8945-11ee-b789-b05cdad83c7f', '021599a1-8944-11ee-b789-b05cdad83c7f', '084bd1d1-8945-11ee-b789-b05cdad83c7f', 'In-transit', 0, 0, 0, 0, 0, 0, 0, 0, 0, '', '', 0, 0),
('bd5bee42-894b-11ee-b789-b05cdad83c7f', 'VN1236HN', 'Tâm', '2023-11-01', '0937128323', 'Tầng 1, Tòa nhà Báo Lao Động, 06 P. Phạm Văn Bạch, Yên Hoà, Cầu Giấy, Hà Nội', '0215a62a-8944-11ee-b789-b05cdad83c7f', 'Trung', '0314912333', '73-75 Thủ Khoa Huân, Phường Bến Thành, Quận 1, Thành phố Hồ Chí Minh 70000, Việt Nam', '1c54efa7-8945-11ee-b789-b05cdad83c7f', '084bd1d1-8945-11ee-b789-b05cdad83c7f', '084bdf03-8945-11ee-b789-b05cdad83c7f', 'In-transit', 0, 0, 0, 0, 0, 0, 0, 0, 0, '', '', 0, 0);

-- --------------------------------------------------------

--
-- Cấu trúc bảng cho bảng `package_delivery`
--

CREATE TABLE `package_delivery` (
  `delivery_id` char(36) NOT NULL,
  `package_id` char(36) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Đang đổ dữ liệu cho bảng `package_delivery`
--

INSERT INTO `package_delivery` (`delivery_id`, `package_id`) VALUES
('cafcbaa5-a563-11ee-8a45-b05cdad83c7f', 'bd5bee42-894b-11ee-b789-b05cdad83c7f');

-- --------------------------------------------------------

--
-- Cấu trúc bảng cho bảng `points`
--

CREATE TABLE `points` (
  `id` char(36) NOT NULL,
  `location` varchar(255) DEFAULT NULL,
  `type` tinyint(1) DEFAULT NULL COMMENT '0 là điểm giao dịch, 1 là điểm tập kết',
  `link_point_id` char(36) DEFAULT NULL,
  `create_date` date DEFAULT NULL,
  `reference` varchar(36) DEFAULT NULL,
  `name` varchar(255) DEFAULT NULL,
  `city` varchar(255) DEFAULT NULL,
  `zipcode` varchar(255) DEFAULT NULL,
  `phone` varchar(255) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Đang đổ dữ liệu cho bảng `points`
--

INSERT INTO `points` (`id`, `location`, `type`, `link_point_id`, `create_date`, `reference`, `name`, `city`, `zipcode`, `phone`) VALUES
('021599a1-8944-11ee-b789-b05cdad83c7f', 'Hà Nội', 1, '084bd1d1-8945-11ee-b789-b05cdad83c7f', '2023-08-01', 'TK0001', 'Điểm A', 'Hà Nội', '84', '0123123123'),
('0215a62a-8944-11ee-b789-b05cdad83c7f', 'quận Cầu Giấy, Hà Nội', 0, '021599a1-8944-11ee-b789-b05cdad83c7f', '2023-09-11', 'GD0001', 'Điểm B', 'Hà Nội', '84', '0123123123'),
('084bd1d1-8945-11ee-b789-b05cdad83c7f', 'Đà Nẵng', 1, '084bdf03-8945-11ee-b789-b05cdad83c7f', '2023-08-01', 'TK0002', 'Điểm E', 'Đà Nẵng', '85', '0184102423'),
('084bdf03-8945-11ee-b789-b05cdad83c7f', 'Thành phố Hồ Chí Minh', 1, NULL, '2023-08-02', 'TK0003', 'Điểm C', 'TP.Hồ Chí Minh', '86', '0148102410'),
('1c54efa7-8945-11ee-b789-b05cdad83c7f', 'Quận 1, Thành phố Hồ Chí Minh', 0, '084bdf03-8945-11ee-b789-b05cdad83c7f', '2023-09-12', 'GD0002', 'Điểm D', 'TP.Hồ Chí Minh', '86', '0104810423');

--
-- Chỉ mục cho các bảng đã đổ
--

--
-- Chỉ mục cho bảng `cur_point_history`
--
ALTER TABLE `cur_point_history`
  ADD KEY `cur_point_history_point_id_foreign` (`point_id`),
  ADD KEY `cur_point_history_package_id_foreign` (`package_id`);

--
-- Chỉ mục cho bảng `delivery`
--
ALTER TABLE `delivery`
  ADD PRIMARY KEY (`id`),
  ADD KEY `delivery_end_point_foreign` (`end_point`),
  ADD KEY `delivery_start_point_foreign` (`start_point`);

--
-- Chỉ mục cho bảng `employees`
--
ALTER TABLE `employees`
  ADD PRIMARY KEY (`id`),
  ADD KEY `employees_point_id_foreign` (`point_id`);

--
-- Chỉ mục cho bảng `items`
--
ALTER TABLE `items`
  ADD KEY `package_id` (`package_id`);

--
-- Chỉ mục cho bảng `package`
--
ALTER TABLE `package`
  ADD PRIMARY KEY (`id`),
  ADD KEY `package_receive_point_foreign` (`receive_point`),
  ADD KEY `package_cur_point_foreign` (`cur_point`),
  ADD KEY `package_send_point_foreign` (`send_point`),
  ADD KEY `next_point` (`next_point`);

--
-- Chỉ mục cho bảng `package_delivery`
--
ALTER TABLE `package_delivery`
  ADD PRIMARY KEY (`delivery_id`),
  ADD KEY `package_delivery_package_id_foreign` (`package_id`);

--
-- Chỉ mục cho bảng `points`
--
ALTER TABLE `points`
  ADD PRIMARY KEY (`id`),
  ADD KEY `gathering_point` (`link_point_id`);

--
-- Các ràng buộc cho các bảng đã đổ
--

--
-- Các ràng buộc cho bảng `cur_point_history`
--
ALTER TABLE `cur_point_history`
  ADD CONSTRAINT `cur_point_history_package_id_foreign` FOREIGN KEY (`package_id`) REFERENCES `package` (`id`) ON DELETE CASCADE,
  ADD CONSTRAINT `cur_point_history_point_id_foreign` FOREIGN KEY (`point_id`) REFERENCES `points` (`id`) ON DELETE CASCADE;

--
-- Các ràng buộc cho bảng `delivery`
--
ALTER TABLE `delivery`
  ADD CONSTRAINT `delivery_end_point_foreign` FOREIGN KEY (`end_point`) REFERENCES `points` (`id`) ON DELETE CASCADE,
  ADD CONSTRAINT `delivery_start_point_foreign` FOREIGN KEY (`start_point`) REFERENCES `points` (`id`) ON DELETE CASCADE;

--
-- Các ràng buộc cho bảng `employees`
--
ALTER TABLE `employees`
  ADD CONSTRAINT `employees_point_id_foreign` FOREIGN KEY (`point_id`) REFERENCES `points` (`id`) ON DELETE SET NULL;

--
-- Các ràng buộc cho bảng `items`
--
ALTER TABLE `items`
  ADD CONSTRAINT `items_ibfk_1` FOREIGN KEY (`package_id`) REFERENCES `package` (`id`) ON DELETE CASCADE;

--
-- Các ràng buộc cho bảng `package`
--
ALTER TABLE `package`
  ADD CONSTRAINT `package_cur_point_foreign` FOREIGN KEY (`cur_point`) REFERENCES `points` (`id`) ON DELETE SET NULL,
  ADD CONSTRAINT `package_ibfk_1` FOREIGN KEY (`next_point`) REFERENCES `points` (`id`) ON DELETE SET NULL,
  ADD CONSTRAINT `package_receive_point_foreign` FOREIGN KEY (`receive_point`) REFERENCES `points` (`id`) ON DELETE SET NULL,
  ADD CONSTRAINT `package_send_point_foreign` FOREIGN KEY (`send_point`) REFERENCES `points` (`id`) ON DELETE SET NULL;

--
-- Các ràng buộc cho bảng `package_delivery`
--
ALTER TABLE `package_delivery`
  ADD CONSTRAINT `package_delivery_delivery_id_foreign` FOREIGN KEY (`delivery_id`) REFERENCES `delivery` (`id`),
  ADD CONSTRAINT `package_delivery_package_id_foreign` FOREIGN KEY (`package_id`) REFERENCES `package` (`id`);

--
-- Các ràng buộc cho bảng `points`
--
ALTER TABLE `points`
  ADD CONSTRAINT `points_ibfk_1` FOREIGN KEY (`link_point_id`) REFERENCES `points` (`id`) ON DELETE SET NULL;
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
