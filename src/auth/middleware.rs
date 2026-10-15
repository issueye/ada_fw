//! 鉴权中间件。
//!
//! [`require_auth`] 从 Cookie 中读取 JWT 并校验：
//! - 校验通过：将 [`CurrentUser`] 注入请求扩展，放行
//! - 校验失败：
//!   - 页面请求（`Accept` 含 `text/html`）→ `303` 重定向到登录页，并携带 `next` 参数
//!   - API 请求 → `401` JSON

use axum::extract::{Request, State};
use axum::http::{header, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Redirect, Response};
use axum_extra::extract::cookie::CookieJar;

use crate::auth::{verify_token, CurrentUser, TOKEN_COOKIE};
use crate::state::AppState;

/// 判断请求是否期望 HTML 响应。
fn wants_html(req: &Request) -> bool {
    req.headers()
        .get(header::ACCEPT)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.contains("text/html"))
        .unwrap_or(false)
}

/// 鉴权中间件。
pub async fn require_auth(
    State(state): State<AppState>,
    jar: CookieJar,
    mut req: Request,
    next: Next,
) -> Response {
    // 未启用鉴权时直接放行（便于测试与本地调���）。
    if !state.config.auth_enabled {
        return next.run(req).await;
    }

    let token = jar.get(TOKEN_COOKIE).map(|c| c.value().to_string());

    let Some(token) = token else {
        return reject(&req, "缺少登录凭据");
    };

    match verify_token(&state.config.jwt_secret, &token) {
        Ok(claims) => {
            // 注入当前用户，供下游 handler 通过 Extension 提取。
            req.extensions_mut().insert(CurrentUser::from(claims));
            next.run(req).await
        }
        Err(_) => reject(&req, "登录凭据无效或已过期"),
    }
}

/// 构造拒绝响应：页面请求重定向，API 请求返回 401。
fn reject(req: &Request, reason: &str) -> Response {
    if wants_html(req) {
        let next = req.uri().path_and_query().map(|p| p.as_str()).unwrap_or("/admin");
        let target = format!("/admin/login?next={}", urlencode(next));
        Redirect::to(&target).into_response()
    } else {
        (
            StatusCode::UNAUTHORIZED,
            axum::Json(serde_json::json!({
                "error": {
                    "code": 401,
                    "message": reason,
                }
            })),
        )
            .into_response()
    }
}

/// 极简 URL 编码（仅处理查询串中需转义的字符）。
fn urlencode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' | b'/' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urlencode_应保留安全字符() {
        assert_eq!(urlencode("/admin/users"), "/admin/users");
    }

    #[test]
    fn urlencode_应转义特殊字符() {
        assert_eq!(urlencode("a b&c=d"), "a%20b%26c%3Dd");
    }
}
