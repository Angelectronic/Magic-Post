-- phpMyAdmin SQL Dump
-- version 5.2.0
-- https://www.phpmyadmin.net/
--
-- Máy chủ: 127.0.0.1
-- Thời gian đã tạo: Th10 28, 2023 lúc 02:05 AM
-- Phiên bản máy phục vụ: 10.4.27-MariaDB
-- Phiên bản PHP: 8.0.25

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
-- Cấu trúc bảng cho bảng `customers`
--

CREATE TABLE `customers` (
  `id` char(36) NOT NULL,
  `username` varchar(255) NOT NULL,
  `password` varchar(255) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Đang đổ dữ liệu cho bảng `customers`
--

INSERT INTO `customers` (`id`, `username`, `password`) VALUES
('f15630f9-8947-11ee-b789-b05cdad83c7f', 'customer1', '123');

-- --------------------------------------------------------

--
-- Cấu trúc bảng cho bảng `delivery`
--

CREATE TABLE `delivery` (
  `id` char(36) NOT NULL,
  `start_point` char(36) NOT NULL,
  `end_point` char(36) NOT NULL,
  `begin_date` datetime NOT NULL,
  `expected_date` datetime DEFAULT NULL,
  `arrived_date` datetime DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Đang đổ dữ liệu cho bảng `delivery`
--

INSERT INTO `delivery` (`id`, `start_point`, `end_point`, `begin_date`, `expected_date`, `arrived_date`) VALUES
('ff81a663-894c-11ee-b789-b05cdad83c7f', '021599a1-8944-11ee-b789-b05cdad83c7f', '084bd1d1-8945-11ee-b789-b05cdad83c7f', '2023-11-02 22:35:48', '2023-11-03 22:35:48', '2023-11-03 22:35:48');

-- --------------------------------------------------------

--
-- Cấu trúc bảng cho bảng `employees`
--

CREATE TABLE `employees` (
  `id` char(36) NOT NULL,
  `name` varchar(255) NOT NULL,
  `dob` date NOT NULL,
  `position` varchar(255) NOT NULL,
  `point_id` char(36) DEFAULT NULL,
  `username` varchar(255) NOT NULL,
  `password` varchar(255) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Đang đổ dữ liệu cho bảng `employees`
--

INSERT INTO `employees` (`id`, `name`, `dob`, `position`, `point_id`, `username`, `password`) VALUES
('291555e2-8944-11ee-b789-b05cdad83c7f', 'Elon Musk', '2003-11-15', 'CEO', NULL, 'ceo', '123'),
('29156008-8944-11ee-b789-b05cdad83c7f', 'Phạm Văn Hùng', '2000-09-09', 'leader', '0215a62a-8944-11ee-b789-b05cdad83c7f', 'leader1', '123'),
('42d4bd1e-8944-11ee-b789-b05cdad83c7f', 'Huỳnh Thế Công', '2009-01-02', 'leader', '021599a1-8944-11ee-b789-b05cdad83c7f', 'leader2', '123'),
('f1409b35-8945-11ee-b789-b05cdad83c7f', 'Đỗ Mạnh Hoa', '2002-02-15', 'leader', '084bd1d1-8945-11ee-b789-b05cdad83c7f', 'leader3', '123'),
('f140a759-8945-11ee-b789-b05cdad83c7f', 'Thế Quang', '2007-09-09', 'leader', '084bdf03-8945-11ee-b789-b05cdad83c7f', 'leader4', '123'),
('f140b180-8945-11ee-b789-b05cdad83c7f', 'Tuấn Khôi', '2003-08-06', 'leader', '1c54efa7-8945-11ee-b789-b05cdad83c7f', 'leader5', '123');

-- --------------------------------------------------------

--
-- Cấu trúc bảng cho bảng `package`
--

CREATE TABLE `package` (
  `id` char(36) NOT NULL,
  `status` varchar(255) NOT NULL,
  `sender_id` char(36) NOT NULL,
  `sent_date` date NOT NULL,
  `required_date` date DEFAULT NULL,
  `shipped_date` date DEFAULT NULL,
  `sent_point` char(36) NOT NULL,
  `received_point` char(36) NOT NULL,
  `current_point` char(36) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Đang đổ dữ liệu cho bảng `package`
--

INSERT INTO `package` (`id`, `status`, `sender_id`, `sent_date`, `required_date`, `shipped_date`, `sent_point`, `received_point`, `current_point`) VALUES
('bd5bee42-894b-11ee-b789-b05cdad83c7f', 'Packing', 'f15630f9-8947-11ee-b789-b05cdad83c7f', '2023-11-01', '2023-11-23', NULL, '0215a62a-8944-11ee-b789-b05cdad83c7f', '1c54efa7-8945-11ee-b789-b05cdad83c7f', '084bd1d1-8945-11ee-b789-b05cdad83c7f');

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
('ff81a663-894c-11ee-b789-b05cdad83c7f', 'bd5bee42-894b-11ee-b789-b05cdad83c7f');

-- --------------------------------------------------------

--
-- Cấu trúc bảng cho bảng `points`
--

CREATE TABLE `points` (
  `id` char(36) NOT NULL,
  `location` varchar(255) NOT NULL,
  `type` tinyint(1) NOT NULL COMMENT '0 là điểm giao dịch, 1 là điểm tập kết'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Đang đổ dữ liệu cho bảng `points`
--

INSERT INTO `points` (`id`, `location`, `type`) VALUES
('021599a1-8944-11ee-b789-b05cdad83c7f', 'Hà Nội', 1),
('0215a62a-8944-11ee-b789-b05cdad83c7f', 'quận Cầu Giấy, Hà Nội', 0),
('084bd1d1-8945-11ee-b789-b05cdad83c7f', 'Đà Nẵng', 1),
('084bdf03-8945-11ee-b789-b05cdad83c7f', 'Thành phố Hồ Chí Minh', 1),
('1c54efa7-8945-11ee-b789-b05cdad83c7f', 'Quận 1, Thành phố Hồ Chí Minh', 0);

--
-- Chỉ mục cho các bảng đã đổ
--

--
-- Chỉ mục cho bảng `customers`
--
ALTER TABLE `customers`
  ADD PRIMARY KEY (`id`);

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
  ADD KEY `package_sender_id_foreign` (`sender_id`),
  ADD KEY `package_received_point_foreign` (`received_point`),
  ADD KEY `package_current_point_foreign` (`current_point`),
  ADD KEY `package_sent_point_foreign` (`sent_point`);

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
  ADD PRIMARY KEY (`id`);

--
-- Các ràng buộc cho các bảng đã đổ
--

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
  ADD CONSTRAINT `package_current_point_foreign` FOREIGN KEY (`current_point`) REFERENCES `points` (`id`),
  ADD CONSTRAINT `package_received_point_foreign` FOREIGN KEY (`received_point`) REFERENCES `points` (`id`),
  ADD CONSTRAINT `package_sender_id_foreign` FOREIGN KEY (`sender_id`) REFERENCES `customers` (`id`),
  ADD CONSTRAINT `package_sent_point_foreign` FOREIGN KEY (`sent_point`) REFERENCES `points` (`id`);

--
-- Các ràng buộc cho bảng `package_delivery`
--
ALTER TABLE `package_delivery`
  ADD CONSTRAINT `package_delivery_delivery_id_foreign` FOREIGN KEY (`delivery_id`) REFERENCES `delivery` (`id`),
  ADD CONSTRAINT `package_delivery_package_id_foreign` FOREIGN KEY (`package_id`) REFERENCES `package` (`id`);
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
