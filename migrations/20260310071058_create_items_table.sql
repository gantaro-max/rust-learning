-- migrations/xxxx_create_items_table.sql

CREATE TABLE items (
    id SERIAL PRIMARY KEY,           -- 自動で増えるID
    name VARCHAR(255) NOT NULL UNIQUE, -- 商品名（重複禁止）
    price INTEGER NOT NULL,          -- 価格
    stock INTEGER NOT NULL,          -- 在庫数
    category VARCHAR(100) NOT NULL,   -- 分類
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP -- 作成日時
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL UNIQUE,
    user_name VARCHAR(100) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
