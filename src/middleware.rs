//! 通用中间件。
//!
//! [`negotiate_error`]：错误响应内容协商。
//!
//! 当请求期望 HTML（`Accept` 含 `text/html`）但下游返回了 JSON 错误体时，
//! 将其替换为渲染后的 HTML 错误页，避免浏览器直接显示裸 JSON。

use axum::extract::{Request, State};
use axum::http::{header, StatusCode};
use axum::middleware::Next;
use axum::response::{Html, IntoResponse, Response};

use crate::handlers::admin::render_error_page;
use crate::state::AppState;

/// 错误响应内容协商中间件。
pub async fn negotiate_error(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Response {
    // 仅对期望 HTML 的请求生效。
    let wants_html = req
        .headers()
        .get(header::ACCEPT)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.contains("text/html"))
        .unwrap_or(false);

    let res = next.run(req).await;

    if !wants_html || !res.status().is_client_error() && !res.status().is_server_error() {
        return res;
    }

    // 仅处理 JSON 错误体（即由 AppError 生成的响应）。
    let is_json = res
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.contains("application/json"))
        .unwrap_or(false);

    if !is_json {
        return res;
    }

    let status = res.status();
    let message = status_message(status);

    match render_error_page(&state, status.as_u16(), message) {
        Ok(html) => (status, Html(html)).into_response(),
        // 渲染失败时保留原响应，避免二次错误。
        Err(_) => res,
    }
}

/// 状态码对应的默认提示文案。
fn status_message(status: StatusCode) -> &'static str {
    match status {
        StatusCode::BAD_REQUEST => "提交的数据格式不正确，请检查后重试。",
        StatusCode::UNAUTHORIZED => "登录状态已失效，请重新登录。",
        StatusCode::FORBIDDEN => "你没有权限访问该资源。",
        StatusCode::NOT_FOUND => "抱歉，你访问的页面已被移除、重命名，或暂时不可用。",
        StatusCode::PAYLOAD_TOO_LARGE => "提交的内容过大，请精简后重试。",
        _ => "服务器处理请求时发生错误，请稍后重试。",
    }
}
