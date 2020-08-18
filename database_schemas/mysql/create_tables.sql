CREATE DATABASE IF NOT EXISTS `maildata`;

use `maildata`;

CREATE TABLE IF NOT EXISTS `default_quotas` (
  `username` varchar(100) NOT NULL,
  `daily` int(11) NOT NULL DEFAULT '200',
  `monthly` int(11) NOT NULL DEFAULT '2000',
  UNIQUE KEY `default_quotas_username_IDX` (`username`) USING BTREE
) ENGINE=MyISAM DEFAULT CHARSET=utf8mb4;


CREATE TABLE IF NOT EXISTS `current_quotas` (
  `username` varchar(100) CHARACTER SET utf8 NOT NULL,
  `daily` int(11) NOT NULL DEFAULT '0',
  `monthly` int(11) NOT NULL DEFAULT '0',
  UNIQUE KEY `current_quotas_username_IDX` (`username`) USING BTREE
) ENGINE=MyISAM DEFAULT CHARSET=utf8mb4;

