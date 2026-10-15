//! 路由注册。
//!
//! 将 SSR 页面路由、后台管理路由、JSON API 路由与静态资源服务组装为统一的 [`Router`]。

use axum::routing::{delete, get, patch};
use axum::Router;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use crate::handlers::{admin, api, pages};
use crate::state::AppState;

/// 构建应用路由。
pub fn build_router(state: AppState) -> Router {
    let static_dir = state.config.static_dir.clone();

    Router::new()
        // ---- 示例页面 ----
        .route("/", get(pages::index))
        .route("/about", get(pages::about))
        // ---- 后台管理 ----
        .route("/admin", get(admin::dashboard))
        .route("/admin/users", get(admin::users))
        .route("/admin/forms", get(admin::forms))
        .route("/admin/settings", get(admin::settings))
        .route("/admin/ui-kit", get(admin::ui_kit))
        .route("/admin/login", get(admin::login))
        .route("/admin/404", get(admin::not_found_page))
        // ---- 待办 API ----
        .route("/api/todos", get(api::list_todos).post(api::create_todo))
        .route("/api/todos/:id", delete(api::delete_todo))
        .route("/api/todos/:id/toggle", patch(api::toggle_todo))
        // ---- 用户 API ----
        .route("/api/users", get(api::list_users).post(api::create_user))
        .route("/api/users/:id", delete(api::delete_user))
        // ---- 静态资源 ----
        .nest_service("/static", ServeDir::new(static_dir))
        // ---- 中间件 ----
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
