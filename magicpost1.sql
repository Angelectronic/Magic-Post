-- phpMyAdmin SQL Dump
-- version 5.2.1
-- https://www.phpmyadmin.net/
--
-- Máy chủ: localhost
-- Thời gian đã tạo: Th10 28, 2023 lúc 10:52 AM
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
  `send_ address` varchar(255) DEFAULT NULL,
  `receive_address` varchar(255) DEFAULT NULL,
  `send_phone` varchar(255) DEFAULT NULL,
  `receive_phone` varchar(255) DEFAULT NULL,
  `receive_name` varchar(255) DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Đang đổ dữ liệu cho bảng `package`
--

INSERT INTO `package` (`id`, `send_point`, `receive_point`, `cur_point`, `status`, `send_name`, `send_date`, `required_date`, `shipped_date`, `send_ address`, `receive_address`, `send_phone`, `receive_phone`, `receive_name`) VALUES
('bd5bee42-894b-11ee-b789-b05cdad83c7f', '0215a62a-8944-11ee-b789-b05cdad83c7f', '1c54efa7-8945-11ee-b789-b05cdad83c7f', '084bd1d1-8945-11ee-b789-b05cdad83c7f', 'In Transit', 'Tâm', '2023-11-01', '2023-11-23', NULL, 'Tầng 1, Tòa nhà Báo Lao Động, 06 P. Phạm Văn Bạch, Yên Hoà, Cầu Giấy, Hà Nội', '73-75 Thủ Khoa Huân, Phường Bến Thành, Quận 1, Thành phố Hồ Chí Minh 70000, Việt Nam', '0937128323', '0314912333', 'Trung');

--
-- Chỉ mục cho các bảng đã đổ
--

--
-- Chỉ mục cho bảng `package`
--
ALTER TABLE `package`
  ADD PRIMARY KEY (`id`),
  ADD KEY `package_receive_point_foreign` (`receive_point`),
  ADD KEY `package_cur_point_foreign` (`cur_point`),
  ADD KEY `package_send_point_foreign` (`send_point`);

--
-- Các ràng buộc cho các bảng đã đổ
--

--
-- Các ràng buộc cho bảng `package`
--
ALTER TABLE `package`
  ADD CONSTRAINT `package_cur_point_foreign` FOREIGN KEY (`cur_point`) REFERENCES `points` (`id`),
  ADD CONSTRAINT `package_receive_point_foreign` FOREIGN KEY (`receive_point`) REFERENCES `points` (`id`),
  ADD CONSTRAINT `package_send_point_foreign` FOREIGN KEY (`send_point`) REFERENCES `points` (`id`);
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
