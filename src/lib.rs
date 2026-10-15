//! Ada Framework —— 基于 Rust + Minijinja + Tailwind CSS + Alpine.js 的 SSR 前端框架。
//!
//! 详见 `ARCHITECTURE.md`。

pub mod config;
pub mod error;
pub mod handlers;
pub mod models;
pub mod router;
pub mod state;
pub mod template;

pub use config::{AppConfig, TailwindMode};
pub use error::{AppError, AppResult};
pub use state::AppState;
pub use template::TemplateEngine;

use anyhow::Result;

/// 初始化应用状态（加载配置、构建模板引擎）。
pub fn init_state() -> Result<AppState> {
    let config = AppConfig::from_env();

    TemplateEngine::ensure_dir(&config.template_dir)?;
    let engine = TemplateEngine::new(&config)?;

    Ok(AppState::new(config, engine))
}
