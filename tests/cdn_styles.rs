//! CDN 模式样式内联验证。
//!
//! 目的：确保 CDN 模式下内联的 `<style>` 包含最新令牌与组件类，
//! 防止 `tokens.html` 与 `tokens.css` 脱节导致 UI 错乱。

use ada_fw::config::{AppConfig, TailwindMode};
use ada_fw::template::TemplateEngine;
use serde_json::json;

/// 构建 CDN 模式的模板引擎。
fn cdn_engine() -> TemplateEngine {
    let cfg = AppConfig {
        tailwind_mode: TailwindMode::Cdn,
        ..AppConfig::default()
    };
    TemplateEngine::new(&cfg).expect("引擎创建失败")
}

/// 渲染 tokens.html 片段。
fn render_tokens() -> String {
    cdn_engine()
        .render("partials/tokens.html", json!({}))
        .expect("tokens.html 渲染失败")
}

#[test]
fn cdn_模式应内联设计令牌() {
    let html = render_tokens();
    // 新版靛蓝主色
    assert!(html.contains("--color-primary: #5b5bd6"), "应包含新版主色");
    // 外壳令牌
    assert!(html.contains("--color-topbar-bg"), "应包含顶栏令牌");
    // 数据色令牌
    assert!(html.contains("--color-stat-blue-bg"), "应包含数据色令牌");
    // 布局令牌
    assert!(html.contains("--tabbar-height"), "应包含分段控件高度令牌");
    assert!(html.contains("--content-max-width"), "应包含内容区最大宽度令牌");
    // 圆角与阴影
    assert!(html.contains("--radius-lg: 10px"), "应包含卡片圆角令牌");
    assert!(html.contains("--shadow-md"), "应包含中号阴影令牌");
}

#[test]
fn cdn_模式应内联组件类() {
    let html = render_tokens();
    assert!(html.contains(".topbar"), "应包含顶栏组件类");
    assert!(html.contains(".menu-item"), "应包含菜单项组件类");
    assert!(html.contains(".tab-item"), "应包含页签组件类");
    assert!(html.contains(".stat-card"), "应包含统计卡组件类");
    assert!(html.contains(".card"), "应包含卡片组件类");
}

#[test]
fn cdn_模式不应残留旧版_indigo_主色() {
    let html = render_tokens();
    assert!(
        !html.contains("#4f46e5"),
        "不应残留旧版 Indigo 主色 #4f46e5"
    );
    assert!(
        !html.contains("#eef2ff"),
        "不应残留旧版 Indigo 浅色底 #eef2ff"
    );
}

#[test]
fn cdn_模式应内联_tailwind_配置含新令牌() {
    let html = cdn_engine()
        .render("partials/tailwind-config.html", json!({}))
        .expect("tailwind-config.html 渲染失败");
    assert!(html.contains("topbar:"), "CDN 配置应含 topbar 颜色");
    assert!(html.contains("stat:"), "CDN 配置应含 stat 颜色");
    assert!(html.contains("tabbar:"), "CDN 配置应含 tabbar 间距");
}

#[test]
fn 本地模式不应内联样式() {
    let cfg = AppConfig {
        tailwind_mode: TailwindMode::Local,
        ..AppConfig::default()
    };
    let engine = TemplateEngine::new(&cfg).expect("引擎创建失败");
    let html = engine
        .render("partials/tokens.html", json!({}))
        .expect("渲染失败");
    assert!(!html.contains("<style>"), "本地模式不应内联 style");
    assert!(html.is_empty() || html.trim().is_empty(), "本地模式应输出空片段");
}
