//! 数据持久化层。
//!
//! 基于 `sqlx` + SQLite 提供：
//! - 连接池初始化与迁移执行
//! - [`Store`](crate::state::Store) 的加载与保存
//!
//! 设计取舍：为保持既有同步 API 与 63 个测试不变，
//! 采用「启动时加载到内存 → 变更后异步落库」的策略。
//! 内存 `Store` 仍是唯一事实来源，SQLite 负责持久化。

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Row, SqlitePool};
use std::str::FromStr;

use crate::error::{AppError, AppResult};
use crate::models::{Todo, User, UserRole, UserStatus};
use crate::state::Store;

/// 默认数据库连接串（文件模式，不存在则创建）。
pub const DEFAULT_DATABASE_URL: &str = "sqlite://ada.db?mode=rwc";

/// 创建 SQLite 连接池并执行迁移。
pub async fn init_pool(database_url: &str) -> AppResult<SqlitePool> {
    let options = SqliteConnectOptions::from_str(database_url)
        .map_err(|e| AppError::Internal(format!("数据库连接串非法: {e}")))?
        .create_if_missing(true);

    // 内存数据库（`sqlite::memory:`）在连接池中每个连接都是独立实例，
    // 因此限制为单连接，保证同一内存库被复用。
    let max_connections = if database_url.contains(":memory:") { 1 } else { 5 };

    let pool = SqlitePoolOptions::new()
        .max_connections(max_connections)
        .connect_with(options)
        .await
        .map_err(|e| AppError::Internal(format!("数据库连接失败: {e}")))?;

    // 执行迁移脚本（内嵌，避免运行时依赖文件路径）。
    sqlx::query(include_str!("../../migrations/0001_init.sql"))
        .execute(&pool)
        .await
        .map_err(|e| AppError::Internal(format!("数据库迁移失败: {e}")))?;

    Ok(pool)
}

/// 从数据库加载全部数据到内存 [`Store`]。
///
/// 注意：本函数**不写入**种子数据，仅做读取。
/// 首次初始化请使用 [`init_store`]。
pub async fn load_store(pool: &SqlitePool) -> AppResult<Store> {
    let mut store = Store::default();

    // ---- 用户 ----
    let user_rows = sqlx::query("SELECT id, name, email, role, status, created_at FROM users")
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::Internal(format!("读取用户失败: {e}")))?;

    for row in &user_rows {
        let role = parse_role(row.get::<String, _>("role").as_str());
        let status = parse_status(row.get::<String, _>("status").as_str());
        store.push_user(User::restore(
            row.get::<String, _>("id"),
            row.get::<String, _>("name"),
            row.get::<String, _>("email"),
            role,
            status,
            row.get::<String, _>("created_at"),
        ));
    }

    // ---- 待办 ----
    let todo_rows = sqlx::query("SELECT id, title, done FROM todos")
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::Internal(format!("读取待办失败: {e}")))?;

    for row in &todo_rows {
        store.push_todo(Todo::restore(
            row.get::<String, _>("id"),
            row.get::<String, _>("title"),
            row.get::<i64, _>("done") != 0,
        ));
    }

    Ok(store)
}

/// 初始化存储：加载数据，若为首次启动（表为空）则写入种子数据。
pub async fn init_store(pool: &SqlitePool) -> AppResult<Store> {
    let store = load_store(pool).await?;

    // 仅当用户表与待办表均为空��，视为首次启动。
    if store.user_count() == 0 && store.list().is_empty() {
        let seeded = Store::with_seed();
        save_store(pool, &seeded).await?;
        tracing::info!("数据库为空，已写入种子数据");
        return Ok(seeded);
    }

    Ok(store)
}

/// 将内存 [`Store`] 全量写回数据库（幂等）。
pub async fn save_store(pool: &SqlitePool, store: &Store) -> AppResult<()> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|e| AppError::Internal(format!("开启事务失败: {e}")))?;

    for user in store.list_users() {
        sqlx::query(
            "INSERT OR REPLACE INTO users (id, name, email, role, status, created_at)
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(&user.id)
        .bind(&user.name)
        .bind(&user.email)
        .bind(role_str(user.role))
        .bind(status_str(user.status))
        .bind(&user.created_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Internal(format!("写入用户失败: {e}")))?;
    }

    for todo in store.list() {
        sqlx::query("INSERT OR REPLACE INTO todos (id, title, done) VALUES (?, ?, ?)")
            .bind(&todo.id)
            .bind(&todo.title)
            .bind(if todo.done { 1 } else { 0 })
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(format!("写入待办失败: {e}")))?;
    }

    tx.commit()
        .await
        .map_err(|e| AppError::Internal(format!("提交事务失败: {e}")))?;

    Ok(())
}

/// 删除指定用户（增量落库）。
pub async fn delete_user(pool: &SqlitePool, id: &str) -> AppResult<()> {
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(format!("删除用户失败: {e}")))?;
    Ok(())
}

/// 删除指定待办（增量落库）。
pub async fn delete_todo(pool: &SqlitePool, id: &str) -> AppResult<()> {
    sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(format!("删除待办失败: {e}")))?;
    Ok(())
}

// ==========================================================================
// 枚举 <-> 字符串
// ==========================================================================

fn role_str(role: UserRole) -> &'static str {
    match role {
        UserRole::Admin => "admin",
        UserRole::Editor => "editor",
        UserRole::Viewer => "viewer",
    }
}

fn status_str(status: UserStatus) -> &'static str {
    match status {
        UserStatus::Active => "active",
        UserStatus::Inactive => "inactive",
        UserStatus::Pending => "pending",
    }
}

fn parse_role(s: &str) -> UserRole {
    match s {
        "admin" => UserRole::Admin,
        "editor" => UserRole::Editor,
        _ => UserRole::Viewer,
    }
}

fn parse_status(s: &str) -> UserStatus {
    match s {
        "active" => UserStatus::Active,
        "pending" => UserStatus::Pending,
        _ => UserStatus::Inactive,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 角色字符串应可往返转换() {
        for role in [UserRole::Admin, UserRole::Editor, UserRole::Viewer] {
            assert_eq!(parse_role(role_str(role)), role);
        }
    }

    #[test]
    fn 状态字符串应可往返转换() {
        for status in [UserStatus::Active, UserStatus::Inactive, UserStatus::Pending] {
            assert_eq!(parse_status(status_str(status)), status);
        }
    }

    #[test]
    fn 未知角色应回退为访客() {
        assert_eq!(parse_role("unknown"), UserRole::Viewer);
    }

    #[test]
    fn 未知状态应回退为停用() {
        assert_eq!(parse_status("unknown"), UserStatus::Inactive);
    }

    #[tokio::test]
    async fn 内存数据库应可初始化并加载种子数据() {
        let pool = init_pool("sqlite::memory:").await.expect("初始化失败");
        let store = init_store(&pool).await.expect("初始化失败");
        assert_eq!(store.user_count(), 8, "首次初始化应写入 8 个种子用户");
        assert_eq!(store.list().len(), 3, "首次初始化应写入 3 条种子待办");
    }

    #[tokio::test]
    async fn 保存后重新加载应保持数据一致() {
        let pool = init_pool("sqlite::memory:").await.expect("初始化失败");
        let mut store = Store::default();
        store.add_user("测试用户", "test@example.com", UserRole::Admin);
        store.add("测试待办");
        save_store(&pool, &store).await.expect("保存失败");

        let loaded = load_store(&pool).await.expect("加载失败");
        assert_eq!(loaded.user_count(), 1);
        assert_eq!(loaded.list().len(), 1);
        assert_eq!(loaded.list_users()[0].name, "测试用户");
        assert_eq!(loaded.list()[0].title, "测试待办");
    }

    #[tokio::test]
    async fn 删除用户后重新加载应不再包含该用户() {
        let pool = init_pool("sqlite::memory:").await.expect("初始化失败");
        // 先写入种子数据，再删除其中一个用户。
        let store = init_store(&pool).await.expect("初始化失败");
        let target = store.list_users()[0].clone();

        delete_user(&pool, &target.id).await.expect("删除失败");
        let loaded = load_store(&pool).await.expect("加载失败");
        assert_eq!(loaded.user_count(), 7, "删除后应剩 7 个用户");
        assert!(
            !loaded.list_users().iter().any(|u| u.id == target.id),
            "被删除的用户不应再出现"
        );
    }

    #[tokio::test]
    async fn 清空数据后重新加载不应重新写入种子() {
        let pool = init_pool("sqlite::memory:").await.expect("初始化失败");
        let store = init_store(&pool).await.expect("初始化失败");
        // 删除全部用户与待办。
        for u in store.list_users() {
            delete_user(&pool, &u.id).await.expect("删除用户失败");
        }
        for t in store.list() {
            delete_todo(&pool, &t.id).await.expect("删除待办失败");
        }

        // load_store 不应重新写入种子数据。
        let loaded = load_store(&pool).await.expect("加载失败");
        assert_eq!(loaded.user_count(), 0);
        assert_eq!(loaded.list().len(), 0);
    }
}
