//! 后台管理页面处理器。
//!
//! 负责组装后台页面上下文并调用布局模板渲染。

use axum::extract::State;
use axum::response::{Html, IntoResponse};
use axum::Json;
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use serde::Serialize;

use crate::error::{AppError, AppResult};
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
    /// 所属分组：相邻且同组的项会归入同一分组标题下。
    pub group: Option<&'static str>,
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

/// 构建侧边栏菜单（按 `group` 分三组），标记当前激活项。
pub fn build_menu(active: &str) -> Vec<MenuItem> {
    let item = |label, href, icon, group| MenuItem {
        label,
        href,
        icon,
        active: href == active,
        group: Some(group),
        children: vec![],
    };

    vec![
        item("仪表盘", "/admin", ICON_DASHBOARD, "概览"),
        item("用户管理", "/admin/users", ICON_USERS, "管理"),
        item("表单示例", "/admin/forms", ICON_FORM, "管理"),
        item("组件库", "/admin/ui-kit", ICON_COMPONENTS, "系统"),
        item("设置", "/admin/settings", ICON_SETTINGS, "系统"),
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

const ICON_DASHBOARD: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="7.5" height="9.5" rx="2"/><rect x="13.5" y="3" width="7.5" height="6" rx="2"/><rect x="13.5" y="12.5" width="7.5" height="8.5" rx="2"/><rect x="3" y="16" width="7.5" height="5" rx="2"/></svg>"#;
const ICON_USERS: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M22 21v-2a4 4 0 0 0-3-3.87M16 3.13a4 4 0 0 1 0 7.75"/></svg>"#;
const ICON_FORM: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><path d="M14 2v6h6M16 13H8M16 17H8M10 9H8"/></svg>"#;
const ICON_COMPONENTS: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><path d="m12 2.5 8.5 4.75v9.5L12 21.5 3.5 16.75v-9.5z"/><path d="M12 21.5v-9.25M3.5 7.25 12 12.25l8.5-5"/></svg>"#;
const ICON_SETTINGS: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 1 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.6 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.6h.09A1.65 1.65 0 0 0 10.1 3.09V3a2 2 0 1 1 4 0v.09A1.65 1.65 0 0 0 15.1 4.6a1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9v.09a1.65 1.65 0 0 0 1.51 1H21a2 2 0 1 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>"#;

/// 供页面复用的图标。
pub mod icons {
    pub const USERS: &str = super::ICON_USERS;
    pub const ACTIVITY: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><path d="M22 12h-4l-3 9L9 3l-3 9H2"/></svg>"#;
    pub const TRENDING: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><path d="m22 6-9.5 9.5-5-5L1 17"/><path d="M17 6h6v6"/></svg>"#;
    pub const WALLET: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><path d="M20 7H5a2 2 0 0 1 0-4h13v4"/><path d="M3 5v14a2 2 0 0 0 2 2h15a1 1 0 0 0 1-1V8a1 1 0 0 0-1-1"/><path d="M17 13.5h.01"/></svg>"#;
}

// ==========================================================================
// 图表数据结构
// ==========================================================================

/// 图表整体尺寸（viewBox 坐标系，与 chart 组件中的 height 保持一致）。
const CHART_W: i32 = 720;
const CHART_H: i32 = 240;
/// 绘图区上下留白，避免曲线贴边。
const CHART_PAD: i32 = 12;

/// Y 轴刻度：`y` 为 viewBox 内像素坐标，`top_pct` 供 HTML 标签绝对定位使用。
#[derive(Serialize)]
struct AxisTick {
    text: String,
    y: i32,
    top_pct: f64,
}

/// 单条数据系列（已预计算为 SVG path，模板只负责渲染）。
#[derive(Serialize)]
struct ChartSeries {
    name: String,
    variant: &'static str,
    line_path: String,
    area_path: String,
}

/// 面积折线图几何数据。
#[derive(Serialize)]
struct Chart {
    width: i32,
    height: i32,
    legend: bool,
    x_labels: Vec<String>,
    y_axis: Vec<AxisTick>,
    series: Vec<ChartSeries>,
}

/// 按 `0 ..= y_max` 均匀取 `steps` 段刻度，金额以「万」为单位取整。
fn format_tick(value: f64) -> String {
    if value.abs() < f64::EPSILON {
        "0".to_string()
    } else if value >= 10_000.0 {
        format!("{}万", (value / 10_000.0).round() as i64)
    } else {
        format!("{}", value.round() as i64)
    }
}

/// 依据数值序列计算折线与面积的 SVG path。
///
/// 所有系列共用同一条 Y 轴（`y_max`），因此调用方必须保证各系列单位一致。
fn build_chart(x_labels: &[&str], y_max: f64, steps: usize, series: Vec<(&str, &'static str, Vec<f64>)>) -> Chart {
    let plot_h = (CHART_H - CHART_PAD * 2) as f64;
    let points = series.iter().map(|s| s.2.len()).max().unwrap_or(0);
    let step_x = if points > 1 {
        CHART_W as f64 / (points - 1) as f64
    } else {
        0.0
    };
    let y_at = |v: f64| CHART_PAD as f64 + plot_h * (1.0 - (v / y_max).clamp(0.0, 1.0));
    let baseline = (CHART_H - CHART_PAD) as f64;

    let built = series
        .into_iter()
        .map(|(name, variant, values)| {
            let mut line = String::new();
            for (i, v) in values.iter().enumerate() {
                if i > 0 {
                    line.push(' ');
                }
                line.push_str(&format!("{}{:.1} {:.1}", if i == 0 { 'M' } else { 'L' }, i as f64 * step_x, y_at(*v)));
            }
            let last_x = values.len().saturating_sub(1) as f64 * step_x;
            let area = format!("{line} L{last_x:.1} {baseline:.1} L0 {baseline:.1} Z");
            ChartSeries {
                name: name.to_string(),
                variant,
                line_path: line,
                area_path: area,
            }
        })
        .collect();

    let y_axis = (0..=steps)
        .map(|i| {
            let ratio = i as f64 / steps as f64;
            let y = CHART_PAD as f64 + plot_h * ratio;
            AxisTick {
                text: format_tick(y_max * (1.0 - ratio)),
                y: y.round() as i32,
                // 保留一位小数即可，避免 HTML 中出现超长浮点串
                top_pct: (y / CHART_H as f64 * 1000.0).round() / 10.0,
            }
        })
        .collect();

    Chart {
        width: CHART_W,
        height: CHART_H,
        legend: true,
        x_labels: x_labels.iter().map(|s| (*s).to_string()).collect(),
        y_axis,
        series: built,
    }
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
    chart: Chart,
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
    caption: &'static str,
    /// 迷你趋势线数据点（viewBox 0 0 100 28）
    spark: &'static str,
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
    /// 组件库中的图表演示数据。
    chart: Chart,
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
            variant: "blue",
            caption: "较上月",
            spark: "0,22 10,20 20,21 30,17 40,18 50,14 60,15 70,11 80,12 90,8 100,6",
        },
        StatItem {
            label: "活跃用户",
            value: store.user_count_by_status(UserStatus::Active).to_string(),
            trend: "+8.2%",
            trend_up: true,
            icon: icons::ACTIVITY,
            variant: "green",
            caption: "较上月",
            spark: "0,18 10,19 20,16 30,17 40,13 50,14 60,11 70,12 80,9 90,10 100,7",
        },
        StatItem {
            label: "待激活",
            value: store.user_count_by_status(UserStatus::Pending).to_string(),
            trend: "-3.1%",
            trend_up: false,
            icon: icons::TRENDING,
            variant: "orange",
            caption: "较上月",
            spark: "0,10 10,12 20,9 30,13 40,11 50,15 60,12 70,16 80,14 90,17 100,15",
        },
        StatItem {
            label: "本月营收",
            value: "¥ 72,300".to_string(),
            trend: "+23.4%",
            trend_up: true,
            icon: icons::WALLET,
            variant: "purple",
            caption: "较上月",
            spark: "0,24 10,21 20,22 30,18 40,19 50,15 60,16 70,12 80,13 90,9 100,5",
        },
    ];

    // 近 12 个月营收（元），两条系列共用同一条 Y 轴。
    let chart = build_chart(
        &[
            "11月", "12月", "1月", "2月", "3月", "4月", "5月", "6月", "7月", "8月", "9月", "10月",
        ],
        80_000.0,
        4,
        vec![
            (
                "本年",
                "blue",
                vec![
                    41200.0, 46800.0, 43600.0, 47900.0, 52400.0, 49600.0, 55100.0, 58300.0, 61200.0,
                    64800.0, 67900.0, 72300.0,
                ],
            ),
            (
                "去年同期",
                "muted",
                vec![
                    35400.0, 38900.0, 37200.0, 40100.0, 43800.0, 42600.0, 46300.0, 48900.0, 51200.0,
                    53400.0, 55800.0, 59100.0,
                ],
            ),
        ],
    );

    let recent_users = users.iter().take(5).cloned().collect();

    let activities = vec![
        Activity { user: "张伟", action: "创建了用户", target: "刘洋", time: "5 分钟前", variant: "success" },
        Activity { user: "李娜", action: "更新了配置", target: "系统设置", time: "23 分钟前", variant: "info" },
        Activity { user: "王强", action: "删除了记录", target: "旧日志", time: "1 小时前", variant: "danger" },
        Activity { user: "赵敏", action: "导出了报表", target: "用户数据", time: "2 小时前", variant: "primary" },
        Activity { user: "周芳", action: "修改了权限", target: "编辑角色", time: "3 小时前", variant: "warning" },
    ];

    let ctx = DashboardContext {
        title: "概览",
        breadcrumb: crumbs(&[("首页", "/admin"), ("概览", "")]),
        menu_items: build_menu("/admin"),
        stats,
        chart,
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
        chart: build_chart(
            &[
                "11月", "12月", "1月", "2月", "3月", "4月", "5月", "6月", "7月", "8月", "9月", "10月",
            ],
            80_000.0,
            4,
            vec![
                (
                    "本年",
                    "blue",
                    vec![
                        41200.0, 46800.0, 43600.0, 47900.0, 52400.0, 49600.0, 55100.0, 58300.0,
                        61200.0, 64800.0, 67900.0, 72300.0,
                    ],
                ),
                (
                    "去年同期",
                    "muted",
                    vec![
                        35400.0, 38900.0, 37200.0, 40100.0, 43800.0, 42600.0, 46300.0, 48900.0,
                        51200.0, 53400.0, 55800.0, 59100.0,
                    ],
                ),
            ],
        ),
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

/// 登录请求体。
#[derive(serde::Deserialize)]
pub struct LoginForm {
    /// 邮箱。
    pub email: String,
    /// 密码。
    pub password: String,
}

/// `POST /admin/login` —— 校验凭据并签发 JWT Cookie。
///
/// 演示实现：任意非空邮箱 + 密码长度 ≥ 6 即视为合法。
/// 生产环境应替换为数据库查询 + 密码哈希校验（如 `argon2`）。
pub async fn do_login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(form): Json<LoginForm>,
) -> AppResult<impl IntoResponse> {
    let email = form.email.trim();
    let password = form.password.trim();

    // 统一错误提示，避免用户枚举。
    let invalid = || AppError::Unauthorized("邮箱或密码错误".to_string());

    if email.is_empty() || !email.contains('@') {
        return Err(invalid());
    }
    if password.len() < 6 {
        return Err(invalid());
    }

    // 从存储中按邮箱查找用户，决定角色；找不到则按访客处理。
    let (user_id, name, role) = {
        let store = state.store.read();
        store
            .list_users()
            .into_iter()
            .find(|u| u.email.eq_ignore_ascii_case(email))
            .map(|u| (u.id, u.name, u.role.label().to_string()))
            .unwrap_or_else(|| (email.to_string(), email.to_string(), "访客".to_string()))
    };

    let token = crate::auth::issue_token(
        &state.config.jwt_secret,
        &user_id,
        &name,
        &role,
        state.config.jwt_ttl_secs,
    )?;

    let cookie = Cookie::build((crate::auth::TOKEN_COOKIE, token))
        .http_only(true)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(time::Duration::seconds(state.config.jwt_ttl_secs as i64))
        .build();

    let jar = jar.add(cookie);
    Ok((jar, Json(serde_json::json!({ "ok": true }))))
}

/// `POST /admin/logout` —— 清除登录 Cookie。
pub async fn do_logout(jar: CookieJar) -> impl IntoResponse {
    let jar = jar.remove(Cookie::from(crate::auth::TOKEN_COOKIE));
    (jar, Json(serde_json::json!({ "ok": true })))
}

/// `GET /admin/404` —— 404 页（独立布局）。
pub async fn not_found_page(State(state): State<AppState>) -> AppResult<Html<String>> {
    let html = render_not_found(&state)?;
    Ok(Html(html))
}

/// 全局兜底处理器：任意未匹配路径均渲染 404 页并返回 `404` 状态码。
pub async fn fallback(State(state): State<AppState>) -> impl axum::response::IntoResponse {
    match render_not_found(&state) {
        Ok(html) => (axum::http::StatusCode::NOT_FOUND, Html(html)).into_response(),
        // 404 页自身渲染失败时降级为纯文本，避免二次错误。
        Err(err) => {
            tracing::error!(error = %err, "渲染 404 页面失败");
            (
                axum::http::StatusCode::NOT_FOUND,
                "404 Not Found",
            )
                .into_response()
        }
    }
}

/// 渲染 404 页面 HTML。
fn render_not_found(state: &AppState) -> AppResult<String> {
    #[derive(Serialize)]
    struct NotFoundContext {
        title: &'static str,
    }

    state.engine.render(
        "admin/404.html",
        NotFoundContext { title: "页面不存在" },
    )
}

/// 渲染通用错误页 HTML（复用 404 模板，展示状态码与提示）。
pub fn render_error_page(state: &AppState, status: u16, message: &str) -> AppResult<String> {
    #[derive(Serialize)]
    struct ErrorContext {
        title: String,
        status: u16,
        heading: String,
        message: String,
    }

    let heading = match status {
        400 => "请求参数有误",
        401 => "需要登录",
        403 => "没有访问权限",
        404 => "页面不存在",
        413 => "请求内容过大",
        500 => "服务器内部错误",
        _ => "出错了",
    };

    state.engine.render(
        "admin/404.html",
        ErrorContext {
            title: format!("{status} 错误"),
            status,
            heading: heading.to_string(),
            message: message.to_string(),
        },
    )
}
