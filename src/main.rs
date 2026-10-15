//! 应用入口。

use ada_fw::{init_state, router::build_router};
use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志。
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 初始化状态。
    let state = init_state().await?;
    let addr = state.config.bind_addr();
    let tailwind = state.config.tailwind_mode.as_str();
    let auth = if state.config.auth_enabled { "on" } else { "off" };
    let db = if state.pool.is_some() { "sqlite" } else { "memory" };

    let app = build_router(state);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!(
        "Ada Framework 已启动: http://{addr}  (tailwind={tailwind}, auth={auth}, store={db})"
    );

    axum::serve(listener, app).await?;
    Ok(())
}
