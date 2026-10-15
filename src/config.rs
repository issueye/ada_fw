//! 应用配置层。
//!
//! 集中管理运行参数，支持通过环境变量覆盖默认值。

use std::path::PathBuf;

/// Tailwind CSS 的集成模式。
///
/// 用于实现「开发期走 CDN、生产期走本地构建产物」的平滑切换。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TailwindMode {
    /// 通过 CDN 引入，零配置，适合开发期。
    Cdn,
    /// 引入本地构建产物 `/static/css/app.css`，适合生产期。
    Local,
}

impl TailwindMode {
    /// 从字符串解析，无法识别时回退到 [`TailwindMode::Cdn`]。
    pub fn from_str_lossy(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "local" | "build" | "prod" | "production" => TailwindMode::Local,
            _ => TailwindMode::Cdn,
        }
    }

    /// 模板中使用的字符串表示。
    pub fn as_str(&self) -> &'static str {
        match self {
            TailwindMode::Cdn => "cdn",
            TailwindMode::Local => "local",
        }
    }
}

/// 应用全局配置。
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// 监听地址。
    pub host: String,
    /// 监听端口。
    pub port: u16,
    /// 模板目��。
    pub template_dir: PathBuf,
    /// 静态资源目录。
    pub static_dir: PathBuf,
    /// Tailwind 集成模式。
    pub tailwind_mode: TailwindMode,
    /// 是否开启模板热重载（开发模式）。
    pub hot_reload: bool,
    /// JWT 签名密钥。
    ///
    /// 生产环境**必须**通过 `ADA_JWT_SECRET` 注入；未设置时使用开发默认值并告警。
    pub jwt_secret: String,
    /// Token 有效期（秒），默认 24 小时。
    pub jwt_ttl_secs: u64,
    /// 是否启用鉴权中间件（默认启用）。
    pub auth_enabled: bool,
    /// 数据库连接串；为 `None` 时使用纯内存存储。
    pub database_url: Option<String>,
}

/// 开发环境默认密钥（仅用于本地调试，生产必须覆盖）。
pub const DEV_JWT_SECRET: &str = "ada-framework-dev-secret-change-me";

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3000,
            template_dir: PathBuf::from("templates"),
            static_dir: PathBuf::from("static"),
            tailwind_mode: TailwindMode::Cdn,
            hot_reload: false,
            jwt_secret: DEV_JWT_SECRET.to_string(),
            jwt_ttl_secs: 24 * 60 * 60,
            auth_enabled: true,
            database_url: None,
        }
    }
}

impl AppConfig {
    /// 从环境变量构建配置，未设置的项使用默认值。
    ///
    /// 支持的环境变量：
    /// - `ADA_HOST`
    /// - `ADA_PORT`
    /// - `ADA_TEMPLATE_DIR`
    /// - `ADA_STATIC_DIR`
    /// - `ADA_TAILWIND_MODE`（`cdn` | `local`）
    /// - `ADA_HOT_RELOAD`（`1` / `true` 开启）
    pub fn from_env() -> Self {
        let mut cfg = Self::default();

        if let Ok(v) = std::env::var("ADA_HOST") {
            cfg.host = v;
        }
        if let Ok(v) = std::env::var("ADA_PORT") {
            if let Ok(port) = v.parse() {
                cfg.port = port;
            }
        }
        if let Ok(v) = std::env::var("ADA_TEMPLATE_DIR") {
            cfg.template_dir = PathBuf::from(v);
        }
        if let Ok(v) = std::env::var("ADA_STATIC_DIR") {
            cfg.static_dir = PathBuf::from(v);
        }
        if let Ok(v) = std::env::var("ADA_TAILWIND_MODE") {
            cfg.tailwind_mode = TailwindMode::from_str_lossy(&v);
        }
        if let Ok(v) = std::env::var("ADA_HOT_RELOAD") {
            cfg.hot_reload = matches!(v.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes");
        }
        if let Ok(v) = std::env::var("ADA_JWT_SECRET") {
            if !v.trim().is_empty() {
                cfg.jwt_secret = v;
            }
        }
        if let Ok(v) = std::env::var("ADA_JWT_TTL_SECS") {
            if let Ok(secs) = v.parse() {
                cfg.jwt_ttl_secs = secs;
            }
        }
        if let Ok(v) = std::env::var("ADA_AUTH_ENABLED") {
            cfg.auth_enabled = !matches!(v.trim().to_ascii_lowercase().as_str(), "0" | "false" | "no");
        }
        if let Ok(v) = std::env::var("ADA_DATABASE_URL") {
            if !v.trim().is_empty() {
                cfg.database_url = Some(v);
            }
        }

        cfg
    }

    /// 校验配置安全性，返回致命问题列表。
    ///
    /// 生产环境（`tailwind_mode == Local` 或显式设置 `ADA_JWT_SECRET`）下
    /// 若仍使用开发默认密钥，视为致命错误。
    pub fn validate(&self) -> Result<(), String> {
        if self.jwt_secret == DEV_JWT_SECRET {
            tracing::warn!(
                "正在使用开发默认 JWT 密钥，请在生产环境通过 ADA_JWT_SECRET 注入强随机密钥"
            );
        }
        if self.jwt_secret.len() < 16 {
            return Err("JWT 密钥长度不足 16 字符，存在被暴力破解风险".to_string());
        }
        Ok(())
    }

    /// 监听地址字符串，形如 `127.0.0.1:3000`。
    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
