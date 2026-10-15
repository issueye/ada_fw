//! HTTP 层集成测试。
//!
//! 通过 `tower::ServiceExt::oneshot` 直接驱动 `Router`，
//! 覆盖路由匹配、状态码、鉴权与响应头契约。

use ada_fw::config::AppConfig;
use ada_fw::router::build_router;
use ada_fw::state::AppState;
use ada_fw::template::TemplateEngine;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::util::ServiceExt;

/// 构建测试用应用实例（鉴权开启）。
fn app() -> axum::Router {
    let config = AppConfig::default();
    let engine = TemplateEngine::new(&config).expect("模板引擎创建失败");
    build_router(AppState::new(config, engine))
}

/// 构建测试用应用实例（鉴权关闭，便于测试业务逻辑）。
fn app_no_auth() -> axum::Router {
    let config = AppConfig {
        auth_enabled: false,
        ..AppConfig::default()
    };
    let engine = TemplateEngine::new(&config).expect("模板引擎创建失败");
    build_router(AppState::new(config, engine))
}

/// 发送 GET 请求。
async fn get(router: axum::Router, uri: &str) -> axum::response::Response {
    router
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .expect("请求失败")
}

/// 发送 JSON POST 请求。
async fn post_json(router: axum::Router, uri: &str, body: &str) -> axum::response::Response {
    router
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(uri)
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .expect("请求失败")
}

/// 登录并返回 Set-Cookie 中的 Token Cookie 值。
async fn login_cookie() -> String {
    let res = post_json(
        app(),
        "/admin/login",
        r#"{"email":"zhangwei@example.com","password":"secret123"}"#,
    )
    .await;
    assert_eq!(res.status(), StatusCode::OK, "登录应成功");

    res.headers()
        .get("set-cookie")
        .and_then(|v| v.to_str().ok())
        .expect("登录响应应包含 Set-Cookie")
        .split(';')
        .next()
        .unwrap()
        .to_string()
}

// ==========================================================================
// 公开路由
// ==========================================================================

#[tokio::test]
async fn 未知路径应返回_404_且为_html() {
    let res = get(app(), "/this-path-does-not-exist").await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND, "未知路径应返回 404");

    let ct = res
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default()
        .to_string();
    assert!(ct.contains("text/html"), "404 应返回 HTML 页面，实际: {ct}");
}

#[tokio::test]
async fn 首页应返回_200_且为_html() {
    let res = get(app(), "/").await;
    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers()["content-type"].to_str().unwrap().contains("text/html"));
}

#[tokio::test]
async fn 登录页应可匿名访问() {
    let res = get(app(), "/admin/login").await;
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn 响应应包含安全头() {
    let res = get(app(), "/").await;
    assert_eq!(res.headers()["x-content-type-options"], "nosniff");
    assert_eq!(res.headers()["x-frame-options"], "DENY");
}

// ==========================================================================
// 鉴权
// ==========================================================================

#[tokio::test]
async fn 未登录访问后台应重定向到登录页() {
    let res = app()
        .oneshot(
            Request::builder()
                .uri("/admin")
                .header("accept", "text/html")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::SEE_OTHER, "应返回 303 重定向");
    let location = res.headers()["location"].to_str().unwrap();
    assert!(location.starts_with("/admin/login"), "实际: {location}");
}

#[tokio::test]
async fn 未登录访问_api_应返回_401() {
    let res = get(app(), "/api/users").await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn 携带有效_token_应可访问后台() {
    let cookie = login_cookie().await;
    let res = app()
        .oneshot(
            Request::builder()
                .uri("/admin")
                .header("cookie", cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn 携带有效_token_应可访问_api() {
    let cookie = login_cookie().await;
    let res = app()
        .oneshot(
            Request::builder()
                .uri("/api/users")
                .header("cookie", cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn 篡改的_token_应被拒绝() {
    let res = app()
        .oneshot(
            Request::builder()
                .uri("/api/users")
                .header("cookie", "ada_token=tampered.token.value")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn 登录密码过短应返回_401() {
    let res = post_json(
        app(),
        "/admin/login",
        r#"{"email":"a@b.com","password":"123"}"#,
    )
    .await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn 登录邮箱非法应返回_401() {
    let res = post_json(
        app(),
        "/admin/login",
        r#"{"email":"not-an-email","password":"secret123"}"#,
    )
    .await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

// ==========================================================================
// 业务 API（关闭鉴权，聚焦业务逻辑）
// ==========================================================================

#[tokio::test]
async fn 待办_api_应返回_json_数组() {
    let res = get(app_no_auth(), "/api/todos").await;
    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers()["content-type"].to_str().unwrap().contains("application/json"));
}

#[tokio::test]
async fn 用户_api_应返回_json_数组() {
    let res = get(app_no_auth(), "/api/users").await;
    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers()["content-type"].to_str().unwrap().contains("application/json"));
}

#[tokio::test]
async fn 删除不存在的待办应返回_404() {
    let res = app_no_auth()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/todos/not-exist")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn 新建待办标题为空应返回_400() {
    let res = post_json(app_no_auth(), "/api/todos", r#"{"title":"   "}"#).await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn 新建待办合法应返回_201() {
    let res = post_json(app_no_auth(), "/api/todos", r#"{"title":"集成测试待办"}"#).await;
    assert_eq!(res.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn 新建用户邮箱非法应返回_400() {
    let res = post_json(
        app_no_auth(),
        "/api/users",
        r#"{"name":"张三","email":"invalid"}"#,
    )
    .await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn 超大请求体应被拒绝() {
    // 构造超过 64 KiB 的请求体。
    let big = "a".repeat(70 * 1024);
    let body = format!(r#"{{"title":"{big}"}}"#);
    let res = post_json(app_no_auth(), "/api/todos", &body).await;
    assert_eq!(
        res.status(),
        StatusCode::PAYLOAD_TOO_LARGE,
        "超大请求体应返回 413"
    );
}

// ==========================================================================
// 错误响应内容协商
// ==========================================================================

#[tokio::test]
async fn 期望_html_的错误请求应返回_html_错误页() {
    // 业务校验失败（400）+ 期望 HTML → 中间件应把 JSON 错误体转为 HTML 错误页。
    let res = app_no_auth()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/todos")
                .header("content-type", "application/json")
                .header("accept", "text/html")
                .body(Body::from(r#"{"title":"   "}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    let ct = res.headers()["content-type"].to_str().unwrap();
    assert!(ct.contains("text/html"), "应返回 HTML 错误页，实际: {ct}");
}

#[tokio::test]
async fn 期望_json_的错误请求应返回_json() {
    let res = app_no_auth()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/todos")
                .header("content-type", "application/json")
                .header("accept", "application/json")
                .body(Body::from(r#"{"title":"   "}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    let ct = res.headers()["content-type"].to_str().unwrap();
    assert!(ct.contains("application/json"), "应返回 JSON，实际: {ct}");
}
