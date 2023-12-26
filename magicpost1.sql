-- phpMyAdmin SQL Dump
-- version 5.2.1
-- https://www.phpmyadmin.net/
--
-- Máy chủ: 127.0.0.1
-- Thời gian đã tạo: Th12 26, 2023 lúc 03:24 PM
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
  `start_point` char(36) DEFAULT NULL,
  `end_point` char(36) DEFAULT NULL,
  `begin_date` datetime DEFAULT NULL,
  `expected_date` datetime DEFAULT NULL,
  `arrived_date` datetime DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- --------------------------------------------------------

--
-- Cấu trúc bảng cho bảng `employees`
--

CREATE TABLE `employees` (
  `id` char(36) NOT NULL,
  `reference` varchar(255) NOT NULL,
  `type` int(11) DEFAULT NULL,
  `create_date` date DEFAULT NULL,
  `last_seen` date DEFAULT NULL,
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

INSERT INTO `employees` (`id`, `reference`, `type`, `create_date`, `last_seen`, `name`, `sex`, `email`, `birthday`, `phone`, `position`, `point_id`, `username`, `password`) VALUES
('1102e612-8ecf-11ee-8c8f-b05cdad83c7f', 'ABC', NULL, '2023-12-08', NULL, 'Phạm Văn Hùng', 'male', NULL, NULL, '', 'leader', '0215a62a-8944-11ee-b789-b05cdad83c7f', 'leader1', '$2b$12$LfPiJzMJQxpJWnba8WreseM8p9h6XudjSZ.emmJB8n7IPvGFkg1b6'),
('285f6ae9-956c-11ee-8651-b05cdad83c7f', 'BDC', NULL, '2023-12-08', NULL, 'hieu', 'male', NULL, NULL, '', 'subordinate', '0215a62a-8944-11ee-b789-b05cdad83c7f', 'hieu2', '$2b$12$VWRvsZlb6U1qbUDEgZlXPeDXDX5.Zxw18Nw38I6rgM6xbFrGhmaxS'),
('36cbbf10-8ecf-11ee-8c8f-b05cdad83c7f', 'UDV', NULL, '2023-12-08', NULL, 'Huỳnh Thế Công', 'male', NULL, NULL, '', 'leader', '021599a1-8944-11ee-b789-b05cdad83c7f', 'leader2', '$2b$12$/fo3UJI/srr6W5f3vNjGxeoPsUlnuWMlWLVhBkdAFXwNh67geUh4y'),
('485cf9b3-8ecf-11ee-8c8f-b05cdad83c7f', 'DEF', NULL, '2023-12-08', NULL, 'Đỗ Mạnh Hoa', 'male', NULL, NULL, '', 'leader', '084bd1d1-8945-11ee-b789-b05cdad83c7f', 'leader3', '$2b$12$IYzrthDGj8lQ35AWfusYtuYoD.bmVBiLANJjP6oKbYNtKFdgWF5Gy'),
('594a6490-8ecf-11ee-8c8f-b05cdad83c7f', 'XYZ', NULL, '2023-12-08', NULL, 'Thế Quang', 'male', NULL, NULL, '', 'leader', '084bdf03-8945-11ee-b789-b05cdad83c7f', 'leader4', '$2b$12$4dxKofFmHXaZhsSwLkKIC.u8GogUC2lhDv0Z6WNW0E1RQFwtQV0ji'),
('69c0a3df-8ecf-11ee-8c8f-b05cdad83c7f', 'POI', NULL, '2023-12-08', NULL, 'Tuấn Khôi', 'male', NULL, NULL, '', 'leader', '1c54efa7-8945-11ee-b789-b05cdad83c7f', 'leader5', '$2b$12$2ewviyB8lCE230QY5e9t8.wagyx/cVJTqqZ9HfeBTI7R.r3nYiXJi'),
('9f2d9992-8ece-11ee-8c8f-b05cdad83c7f', 'ELO', NULL, '2023-12-08', NULL, 'Elon Musk', 'male', NULL, NULL, '', 'CEO', NULL, 'ceo', '$2b$12$5i9Tvx5gf2W1y96IrEV7vO.DLZYEJnU8PyP.N1c5jJiRWiJuBNSVq');

-- --------------------------------------------------------

--
-- Cấu trúc bảng cho bảng `package`
--

CREATE TABLE `package` (
  `id` char(36) NOT NULL,
  `send_point` char(36) DEFAULT NULL,
  `receive_point` char(36) DEFAULT NULL,
  `cur_point` char(36) DEFAULT NULL,
  `status` varchar(255) DEFAULT NULL,
  `send_name` varchar(255) DEFAULT NULL,
  `send_date` date DEFAULT NULL,
  `required_date` date DEFAULT NULL,
  `shipped_date` date DEFAULT NULL,
  `send_address` varchar(255) DEFAULT NULL,
  `receive_address` varchar(255) DEFAULT NULL,
  `send_phone` varchar(255) DEFAULT NULL,
  `receive_phone` varchar(255) DEFAULT NULL,
  `receive_name` varchar(255) DEFAULT NULL,
  `next_point` char(36) DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Đang đổ dữ liệu cho bảng `package`
--

INSERT INTO `package` (`id`, `send_point`, `receive_point`, `cur_point`, `status`, `send_name`, `send_date`, `required_date`, `shipped_date`, `send_address`, `receive_address`, `send_phone`, `receive_phone`, `receive_name`, `next_point`) VALUES
('0144fd37-957b-11ee-90fa-b05cdad83c7f', '0215a62a-8944-11ee-b789-b05cdad83c7f', '1c54efa7-8945-11ee-b789-b05cdad83c7f', '084bd1d1-8945-11ee-b789-b05cdad83c7f', 'In-transit', 'Tâm', '2023-11-01', '2023-11-23', '0000-00-00', 'Tầng 1, Tòa nhà Báo Lao Động, 06 P. Phạm Văn Bạch, Yên Hoà, Cầu Giấy, Hà Nội', '73-75 Thủ Khoa Huân, Phường Bến Thành, Quận 1, Thành phố Hồ Chí Minh 70000, Việt Nam', '0937128323', '0314912333', 'Nam', '1c54efa7-8945-11ee-b789-b05cdad83c7f'),
('bd5bee42-894b-11ee-b789-b05cdad83c7f', '0215a62a-8944-11ee-b789-b05cdad83c7f', '1c54efa7-8945-11ee-b789-b05cdad83c7f', '084bd1d1-8945-11ee-b789-b05cdad83c7f', 'In-transit', 'Tâm', '2023-11-01', '2023-11-23', NULL, 'Tầng 1, Tòa nhà Báo Lao Động, 06 P. Phạm Văn Bạch, Yên Hoà, Cầu Giấy, Hà Nội', '73-75 Thủ Khoa Huân, Phường Bến Thành, Quận 1, Thành phố Hồ Chí Minh 70000, Việt Nam', '0937128323', '0314912333', 'Trung', '021599a1-8944-11ee-b789-b05cdad83c7f');

-- --------------------------------------------------------

--
-- Cấu trúc bảng cho bảng `package_delivery`
--

CREATE TABLE `package_delivery` (
  `delivery_id` char(36) NOT NULL,
  `package_id` char(36) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

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
('0a1a4f59-a3f7-11ee-90a4-b05cdad83c7f', 'Test', 1, NULL, '2023-12-26', 'TK0002', 'Điểm F', 'test', 'test', '0210381414'),
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
  ADD CONSTRAINT `cur_point_history_package_id_foreign` FOREIGN KEY (`package_id`) REFERENCES `package` (`id`),
  ADD CONSTRAINT `cur_point_history_point_id_foreign` FOREIGN KEY (`point_id`) REFERENCES `points` (`id`);

--
-- Các ràng buộc cho bảng `delivery`
--
ALTER TABLE `delivery`
  ADD CONSTRAINT `delivery_end_point_foreign` FOREIGN KEY (`end_point`) REFERENCES `points` (`id`),
  ADD CONSTRAINT `delivery_start_point_foreign` FOREIGN KEY (`start_point`) REFERENCES `points` (`id`);

--
-- Các ràng buộc cho bảng `employees`
--
ALTER TABLE `employees`
  ADD CONSTRAINT `employees_point_id_foreign` FOREIGN KEY (`point_id`) REFERENCES `points` (`id`);

--
-- Các ràng buộc cho bảng `package`
--
ALTER TABLE `package`
  ADD CONSTRAINT `package_cur_point_foreign` FOREIGN KEY (`cur_point`) REFERENCES `points` (`id`),
  ADD CONSTRAINT `package_ibfk_1` FOREIGN KEY (`next_point`) REFERENCES `points` (`id`),
  ADD CONSTRAINT `package_receive_point_foreign` FOREIGN KEY (`receive_point`) REFERENCES `points` (`id`),
  ADD CONSTRAINT `package_send_point_foreign` FOREIGN KEY (`send_point`) REFERENCES `points` (`id`);

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
  ADD CONSTRAINT `points_ibfk_1` FOREIGN KEY (`link_point_id`) REFERENCES `points` (`id`);
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
