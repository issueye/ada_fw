//! Ada Framework —— 基于 Rust + Minijinja + Tailwind CSS + Alpine.js 的 SSR 前端框架。
//!
//! 详见 `ARCHITECTURE.md`。

pub mod auth;
pub mod config;
pub mod db;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod router;
pub mod state;
pub mod template;

pub use config::{AppConfig, TailwindMode};
pub use error::{AppError, AppResult};
pub use state::AppState;
pub use template::TemplateEngine;

use anyhow::Result;

/// 初始化应用状态（加载配置、构建模板引擎、按需连接数据库）。
pub async fn init_state() -> Result<AppState> {
    let config = AppConfig::from_env();

    // 配置安全校验：密钥过短等致命问题直接启动失败（fail-close）。
    config.validate().map_err(anyhow::Error::msg)?;

    TemplateEngine::ensure_dir(&config.template_dir)?;
    let engine = TemplateEngine::new(&config)?;

    // 开发模式：启用模板热重载。
    if config.hot_reload {
        engine.watch(&config)?;
    }

    // 启用持久化时：连接数据库并加载数据。
    match &config.database_url {
        Some(url) => {
            let pool = db::init_pool(url).await?;
            let store = db::init_store(&pool).await?;
            tracing::info!("已启用 SQLite 持久化: {url}");
            Ok(AppState::with_pool(config, engine, store, pool))
        }
        None => Ok(AppState::new(config, engine)),
    }
}
