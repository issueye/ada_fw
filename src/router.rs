//! 路由注册。
//!
//! 将 SSR 页面路由、后台管理路由、JSON API 路由与静态资源服务组装为统一的 [`Router`]。
//!
//! 鉴权策略：
//! - `/admin/login`、`/admin/404`、`/`、`/about`、`/static/*` 免鉴权
//! - 其余 `/admin/*` 与 `/api/*` 经 [`crate::auth::middleware::require_auth`] 校验

use axum::extract::DefaultBodyLimit;
use axum::http::{header, HeaderValue};
use axum::middleware;
use axum::routing::{delete, get, patch, post};
use axum::Router;
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::TraceLayer;

use crate::auth::middleware::require_auth;
use crate::handlers::{admin, api, pages};
use crate::state::AppState;

/// 请求体大小上限（64 KiB）。
const MAX_BODY_BYTES: usize = 64 * 1024;

/// 构建应用路由。
pub fn build_router(state: AppState) -> Router {
    let static_dir = state.config.static_dir.clone();

    // ---- 公开路由（免鉴权） ----
    let public = Router::new()
        .route("/", get(pages::index))
        .route("/about", get(pages::about))
        .route("/admin/login", get(admin::login).post(admin::do_login))
        .route("/admin/logout", post(admin::do_logout))
        .route("/admin/404", get(admin::not_found_page));

    // ---- 受保护的后台页面 ----
    let protected_admin = Router::new()
        .route("/admin", get(admin::dashboard))
        .route("/admin/users", get(admin::users))
        .route("/admin/forms", get(admin::forms))
        .route("/admin/settings", get(admin::settings))
        .route("/admin/ui-kit", get(admin::ui_kit))
        .route_layer(middleware::from_fn_with_state(state.clone(), require_auth));

    // ---- 受保护的 JSON API ----
    let protected_api = Router::new()
        .route("/api/todos", get(api::list_todos).post(api::create_todo))
        .route("/api/todos/:id", delete(api::delete_todo))
        .route("/api/todos/:id/toggle", patch(api::toggle_todo))
        .route("/api/users", get(api::list_users).post(api::create_user))
        .route("/api/users/:id", delete(api::delete_user))
        .route_layer(middleware::from_fn_with_state(state.clone(), require_auth));

    Router::new()
        .merge(public)
        .merge(protected_admin)
        .merge(protected_api)
        // ---- 静态资源 ----
        .nest_service("/static", ServeDir::new(static_dir))
        // ---- 全局兜底：未匹配路径渲染 404 页 ----
        .fallback(admin::fallback)
        // ---- 安全响应头 ----
        .layer(SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::REFERRER_POLICY,
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        ))
        // ---- 请求体大小限制 ----
        .layer(DefaultBodyLimit::max(MAX_BODY_BYTES))
        // ---- 错误响应内容协商（HTML 请求渲染错误页） ----
        .layer(middleware::from_fn_with_state(
            state.clone(),
            crate::middleware::negotiate_error,
        ))
        // ---- 链路追踪 ----
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
