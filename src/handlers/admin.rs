//! 后台管理页面处理器。
//!
//! 负责组装后台页面上下文并调用布局模板渲染。

use axum::extract::State;
use axum::response::Html;
use serde::Serialize;

use crate::error::AppResult;
use crate::models::{User, UserStatus};
use crate::state::AppState;

// ==========================================================================
// 共享上下文
// ==========================================================================

/// 侧边栏菜单项。
#[derive(Serialize, Clone)]
pub struct MenuItem {
    /// 菜单文字。
    pub label: &'static str,
    /// 链接。
    pub href: &'static str,
    /// 图标 SVG。
    pub icon: &'static str,
    /// 是否激活。
    pub active: bool,
    /// 子菜单。
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<MenuItem>,
}

/// 面包屑项。
#[derive(Serialize, Clone)]
pub struct Crumb {
    /// 文字。
    pub label: &'static str,
    /// 链接（最后一项为空）。
    pub href: &'static str,
}

/// 构建侧边栏菜单，标记当前激活项。
pub fn build_menu(active: &str) -> Vec<MenuItem> {
    let item = |label, href, icon| MenuItem {
        label,
        href,
        icon,
        active: href == active,
        children: vec![],
    };

    vec![
        item("仪表盘", "/admin", ICON_DASHBOARD),
        item("用户管理", "/admin/users", ICON_USERS),
        item("表单示例", "/admin/forms", ICON_FORM),
        item("组件库", "/admin/ui-kit", ICON_COMPONENTS),
        item("设置", "/admin/settings", ICON_SETTINGS),
    ]
}

/// 构建面包屑。
pub fn crumbs(items: &[(&'static str, &'static str)]) -> Vec<Crumb> {
    items
        .iter()
        .map(|(label, href)| Crumb { label, href })
        .collect()
}

// ==========================================================================
// 图标常量
// ==========================================================================

const ICON_DASHBOARD: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="7" height="9" rx="1"/><rect x="14" y="3" width="7" height="5" rx="1"/><rect x="14" y="12" width="7" height="9" rx="1"/><rect x="3" y="16" width="7" height="5" rx="1"/></svg>"#;
const ICON_USERS: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><path d="M16 21v-2a4 4 0 00-4-4H6a4 4 0 00-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M22 21v-2a4 4 0 00-3-3.87M16 3.13a4 4 0 010 7.75"/></svg>"#;
const ICON_FORM: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z"/><path d="M14 2v6h6M16 13H8M16 17H8M10 9H8"/></svg>"#;
const ICON_COMPONENTS: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><path d="M12 2l9 5v10l-9 5-9-5V7z"/><path d="M12 22V12M3 7l9 5 9-5"/></svg>"#;
const ICON_SETTINGS: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 00.33 1.82l.06.06a2 2 0 11-2.83 2.83l-.06-.06a1.65 1.65 0 00-1.82-.33 1.65 1.65 0 00-1 1.51V21a2 2 0 11-4 0v-.09A1.65 1.65 0 009 19.4a1.65 1.65 0 00-1.82.33l-.06.06a2 2 0 11-2.83-2.83l.06-.06a1.65 1.65 0 00.33-1.82 1.65 1.65 0 00-1.51-1H3a2 2 0 110-4h.09A1.65 1.65 0 004.6 9a1.65 1.65 0 00-.33-1.82l-.06-.06a2 2 0 112.83-2.83l.06.06a1.65 1.65 0 001.82.33H9a1.65 1.65 0 001-1.51V3a2 2 0 114 0v.09a1.65 1.65 0 001 1.51 1.65 1.65 0 001.82-.33l.06-.06a2 2 0 112.83 2.83l-.06.06a1.65 1.65 0 00-.33 1.82V9a1.65 1.65 0 001.51 1H21a2 2 0 110 4h-.09a1.65 1.65 0 00-1.51 1z"/></svg>"#;

/// 供页面复用的图标。
pub mod icons {
    pub const USERS: &str = super::ICON_USERS;
    pub const ACTIVITY: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><path d="M22 12h-4l-3 9L9 3l-3 9H2"/></svg>"#;
    pub const TRENDING: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><path d="M23 6l-9.5 9.5-5-5L1 18"/><path d="M17 6h6v6"/></svg>"#;
    pub const DOLLAR: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><path d="M12 1v22M17 5H9.5a3.5 3.5 0 000 7h5a3.5 3.5 0 010 7H6"/></svg>"#;
}

// ==========================================================================
// 页面上下文
// ==========================================================================

#[derive(Serialize)]
struct DashboardContext {
    title: &'static str,
    breadcrumb: Vec<Crumb>,
    menu_items: Vec<MenuItem>,
    stats: Vec<StatItem>,
    recent_users: Vec<User>,
    activities: Vec<Activity>,
}

#[derive(Serialize)]
struct StatItem {
    label: &'static str,
    value: String,
    trend: &'static str,
    trend_up: bool,
    icon: &'static str,
    variant: &'static str,
}

#[derive(Serialize)]
struct Activity {
    user: &'static str,
    action: &'static str,
    target: &'static str,
    time: &'static str,
    variant: &'static str,
}

#[derive(Serialize)]
struct UsersContext {
    title: &'static str,
    breadcrumb: Vec<Crumb>,
    menu_items: Vec<MenuItem>,
    users: Vec<User>,
    total: usize,
    active_count: usize,
    pending_count: usize,
    inactive_count: usize,
}

#[derive(Serialize)]
struct FormsContext {
    title: &'static str,
    breadcrumb: Vec<Crumb>,
    menu_items: Vec<MenuItem>,
}

#[derive(Serialize)]
struct SettingsContext {
    title: &'static str,
    breadcrumb: Vec<Crumb>,
    menu_items: Vec<MenuItem>,
}

#[derive(Serialize)]
struct UiKitContext {
    title: &'static str,
    breadcrumb: Vec<Crumb>,
    menu_items: Vec<MenuItem>,
    components: Vec<ComponentGroup>,
}

#[derive(Serialize)]
struct ComponentGroup {
    category: &'static str,
    items: Vec<ComponentInfo>,
}

#[derive(Serialize)]
struct ComponentInfo {
    name: &'static str,
    description: &'static str,
}

// ==========================================================================
// 处理器
// ==========================================================================

/// `GET /admin` —— 仪表盘。
pub async fn dashboard(State(state): State<AppState>) -> AppResult<Html<String>> {
    let store = state.store.read();
    let users = store.list_users();

    let stats = vec![
        StatItem {
            label: "总用户数",
            value: store.user_count().to_string(),
            trend: "+12.5%",
            trend_up: true,
            icon: icons::USERS,
            variant: "primary",
        },
        StatItem {
            label: "活跃用户",
            value: store.user_count_by_status(UserStatus::Active).to_string(),
            trend: "+8.2%",
            trend_up: true,
            icon: icons::ACTIVITY,
            variant: "success",
        },
        StatItem {
            label: "待激活",
            value: store.user_count_by_status(UserStatus::Pending).to_string(),
            trend: "-3.1%",
            trend_up: false,
            icon: icons::TRENDING,
            variant: "warning",
        },
        StatItem {
            label: "本月收入",
            value: "¥ 48,290".to_string(),
            trend: "+23.4%",
            trend_up: true,
            icon: icons::DOLLAR,
            variant: "info",
        },
    ];

    let recent_users = users.iter().take(5).cloned().collect();

    let activities = vec![
        Activity { user: "张伟", action: "创建了用户", target: "刘洋", time: "5 分钟前", variant: "success" },
        Activity { user: "李娜", action: "更新了配置", target: "系统设置", time: "23 分钟前", variant: "info" },
        Activity { user: "王强", action: "删除了记录", target: "旧日志", time: "1 小时前", variant: "danger" },
        Activity { user: "赵敏", action: "导出了报表", target: "用户数据", time: "2 小时前", variant: "primary" },
        Activity { user: "周芳", action: "修改了权限", target: "编辑角色", time: "3 小时前", variant: "warning" },
    ];

    let ctx = DashboardContext {
        title: "仪表盘",
        breadcrumb: crumbs(&[("首页", "/admin"), ("仪表盘", "")]),
        menu_items: build_menu("/admin"),
        stats,
        recent_users,
        activities,
    };

    let html = state.engine.render("admin/dashboard.html", ctx)?;
    Ok(Html(html))
}

/// `GET /admin/users` —— 用户管理。
pub async fn users(State(state): State<AppState>) -> AppResult<Html<String>> {
    let store = state.store.read();

    let ctx = UsersContext {
        title: "用户管理",
        breadcrumb: crumbs(&[("首页", "/admin"), ("用户管理", "")]),
        menu_items: build_menu("/admin/users"),
        users: store.list_users(),
        total: store.user_count(),
        active_count: store.user_count_by_status(UserStatus::Active),
        pending_count: store.user_count_by_status(UserStatus::Pending),
        inactive_count: store.user_count_by_status(UserStatus::Inactive),
    };

    let html = state.engine.render("admin/users.html", ctx)?;
    Ok(Html(html))
}

/// `GET /admin/forms` —— 表单示例。
pub async fn forms(State(state): State<AppState>) -> AppResult<Html<String>> {
    let ctx = FormsContext {
        title: "表单示例",
        breadcrumb: crumbs(&[("首页", "/admin"), ("表单示例", "")]),
        menu_items: build_menu("/admin/forms"),
    };

    let html = state.engine.render("admin/forms.html", ctx)?;
    Ok(Html(html))
}

/// `GET /admin/settings` —— 设置。
pub async fn settings(State(state): State<AppState>) -> AppResult<Html<String>> {
    let ctx = SettingsContext {
        title: "设置",
        breadcrumb: crumbs(&[("首页", "/admin"), ("设置", "")]),
        menu_items: build_menu("/admin/settings"),
    };

    let html = state.engine.render("admin/settings.html", ctx)?;
    Ok(Html(html))
}

/// `GET /admin/ui-kit` —— 组件库展示。
pub async fn ui_kit(State(state): State<AppState>) -> AppResult<Html<String>> {
    let components = crate::template::component::components_by_category()
        .into_iter()
        .map(|(category, items)| ComponentGroup {
            category,
            items: items
                .into_iter()
                .map(|c| ComponentInfo {
                    name: c.name,
                    description: c.description,
                })
                .collect(),
        })
        .collect();

    let ctx = UiKitContext {
        title: "组件库",
        breadcrumb: crumbs(&[("首页", "/admin"), ("组件库", "")]),
        menu_items: build_menu("/admin/ui-kit"),
        components,
    };

    let html = state.engine.render("admin/ui-kit.html", ctx)?;
    Ok(Html(html))
}

/// `GET /admin/login` —— 登录页（独立布局）。
pub async fn login(State(state): State<AppState>) -> AppResult<Html<String>> {
    #[derive(Serialize)]
    struct LoginContext {
        title: &'static str,
    }

    let html = state.engine.render(
        "admin/login.html",
        LoginContext { title: "登录" },
    )?;
    Ok(Html(html))
}

/// `GET /admin/404` —— 404 页（独立布局）。
pub async fn not_found_page(State(state): State<AppState>) -> AppResult<Html<String>> {
    #[derive(Serialize)]
    struct NotFoundContext {
        title: &'static str,
    }

    let html = state.engine.render(
        "admin/404.html",
        NotFoundContext { title: "页面不存在" },
    )?;
    Ok(Html(html))
}
