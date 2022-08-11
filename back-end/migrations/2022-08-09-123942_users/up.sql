-- Your SQL goes here
CREATE TABLE users (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  username VARCHAR(60) NOT NULL UNIQUE,
  first_name NVARCHAR(60),
  last_name NVARCHAR(60),
  email VARCHAR(100),
  password NVARCHAR(150) NOT NULL,
  is_staff BOOLEAN NOT NULL DEFAULT 0 CHECK(is_staff IN (0,1)),
  is_active BOOLEAN NOT NULL DEFAULT 0 CHECK(is_active IN (0,1)),
  is_superuser BOOLEAN NOT NULL DEFAULT 0 CHECK(is_superuser IN (0,1)),
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
--   updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

