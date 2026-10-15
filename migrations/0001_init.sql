-- 初始化表结构。
-- 说明：`users` 与 `todos` 的字段与 `src/models/` 中的领域模型一一对应。

CREATE TABLE IF NOT EXISTS users (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL,
    email       TEXT NOT NULL UNIQUE,
    role        TEXT NOT NULL,
    status      TEXT NOT NULL,
    created_at  TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS todos (
    id     TEXT PRIMARY KEY,
    title  TEXT NOT NULL,
    done   INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
