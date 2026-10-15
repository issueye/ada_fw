//! SSR 页面处理器。
//!
//! 负责组装页面上下文并调用模板引擎渲染完整 HTML。

use axum::extract::State;
use axum::response::Html;
use serde::Serialize;

use crate::error::AppResult;
use crate::state::AppState;

/// 导航项。
#[derive(Serialize)]
struct NavItem {
    label: &'static str,
    href: &'static str,
}

/// 首页上下文。
#[derive(Serialize)]
struct IndexContext {
    title: &'static str,
    nav_items: Vec<NavItem>,
    active: &'static str,
    todos: Vec<crate::models::Todo>,
    /// 初始数据以 JSON 形式注入，供 Alpine 接管。
    initial_todos_json: String,
}

/// 关于页上下文。
#[derive(Serialize)]
struct AboutContext {
    title: &'static str,
    nav_items: Vec<NavItem>,
    active: &'static str,
}

fn nav_items() -> Vec<NavItem> {
    vec![
        NavItem {
            label: "首页",
            href: "/",
        },
        NavItem {
            label: "关于",
            href: "/about",
        },
    ]
}

/// `GET /` —— 首页。
pub async fn index(State(state): State<AppState>) -> AppResult<Html<String>> {
    let todos = state.store.read().list();
    let initial_todos_json = serde_json::to_string(&todos).unwrap_or_else(|_| "[]".to_string());

    let ctx = IndexContext {
        title: "Ada Framework · 首页",
        nav_items: nav_items(),
        active: "/",
        todos,
        initial_todos_json,
    };

    let html = state.engine.render("index.html", ctx)?;
    Ok(Html(html))
}

/// `GET /about` —— 关于页。
pub async fn about(State(state): State<AppState>) -> AppResult<Html<String>> {
    let ctx = AboutContext {
        title: "Ada Framework · 关于",
        nav_items: nav_items(),
        active: "/about",
    };

    let html = state.engine.render("about.html", ctx)?;
    Ok(Html(html))
}
