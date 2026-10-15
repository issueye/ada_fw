//! 模板渲染冒烟测试。
//!
//! 目的：确保所有页面模板在真实上下文下可成功渲染，
//! 捕获模板语法错误、宏参数不匹配、变量缺失等问题。
//!
//! 说明：这些测试依赖工作区根目录下的 `templates/` 目录，
//! 因此需以 crate 根为工作目录运行（`cargo test` 默认满足）。

use ada_fw::config::AppConfig;
use ada_fw::models::{Todo, User, UserRole, UserStatus};
use ada_fw::template::TemplateEngine;
use serde_json::json;

/// 构建测试用模板引擎。
fn engine() -> TemplateEngine {
    TemplateEngine::new(&AppConfig::default()).expect("模板引擎创建失败")
}

/// 校验所有 `x-data` 属性都是完整的 JS 表达式。
///
/// 背景：`x-data` 是双引号属性，若模板把带引号的字符串（例如 `tojson` 的输出）
/// 直接拼进去，属性会在内层引号处提前闭合，Alpine 静默初始化失败——
/// 页面能渲染 200 但交互全部失效，肉眼很难发现。此断言把这类问题挡在测试阶段。
fn assert_x_data_well_formed(html: &str, page: &str) {
    let mut rest = html;
    while let Some(idx) = rest.find("x-data=\"") {
        let after = &rest[idx + "x-data=\"".len()..];
        let end = after
            .find('"')
            .unwrap_or_else(|| panic!("{page}: x-data 属性未闭合"));
        let value = &after[..end];
        let trimmed = value.trim();
        assert!(!trimmed.is_empty(), "{page}: x-data 属性为空");
        assert!(
            trimmed.ends_with('}') || trimmed.ends_with(')'),
            "{page}: x-data 疑似被引号截断，实际取值: {value:?}"
        );
        rest = &after[end..];
    }
}

/// 构造一个完整的后台页面上下文。
fn admin_ctx(title: &str) -> serde_json::Value {
    json!({
        "title": title,
        "breadcrumb": [
            { "label": "首页", "href": "/admin" },
            { "label": title, "href": "" }
        ],
        "menu_items": [
            { "label": "仪表盘", "href": "/admin", "icon": "<svg></svg>", "active": true, "group": "概览" },
            { "label": "用户管理", "href": "/admin/users", "icon": "<svg></svg>", "active": false, "group": "管理" }
        ],
    })
}

/// 构造组件库页所需的图表演示数据。
fn chart_ctx() -> serde_json::Value {
    json!({
        "width": 720,
        "height": 240,
        "legend": true,
        "x_labels": ["1月", "2月"],
        "y_axis": [
            { "text": "8万", "y": 12, "top_pct": 5.0 },
            { "text": "0", "y": 228, "top_pct": 95.0 }
        ],
        "series": [{
            "name": "本年",
            "variant": "blue",
            "line_path": "M0.0 100.0 L50.0 50.0",
            "area_path": "M0.0 100.0 L50.0 50.0 L50.0 228.0 L0.0 228.0 Z"
        }]
    })
}

#[test]
fn 首页模板应可渲染() {
    let todos = vec![Todo::new("测试待办")];
    let ctx = json!({
        "title": "首页",
        "nav_items": [{ "label": "首页", "href": "/" }],
        "active": "/",
        "todos": todos,
        "initial_todos_json": serde_json::to_string(&todos).unwrap(),
    });
    let html = engine().render("index.html", ctx).expect("index.html 渲染失败");
    assert!(html.contains("测试待办"), "首页应包含待办标题");
}

#[test]
fn 关于页模板应可渲染() {
    let ctx = json!({
        "title": "关于",
        "nav_items": [{ "label": "关于", "href": "/about" }],
        "active": "/about",
    });
    engine().render("about.html", ctx).expect("about.html 渲染失败");
}

#[test]
fn 仪表盘模板应可渲染() {
    let ctx = json!({
        "title": "概览",
        "breadcrumb": [{ "label": "概览", "href": "" }],
        "menu_items": [],
        "stats": [{
            "label": "总用户数", "value": "8", "trend": "+1%",
            "trend_up": true, "icon": "<svg></svg>", "variant": "blue",
            "caption": "较上月", "spark": "0,20 50,10 100,5"
        }],
        "chart": chart_ctx(),
        "recent_users": [User::new("张三", "z@b.com", UserRole::Admin, UserStatus::Active, "2024-01-01")],
        "activities": [{
            "user": "张三", "action": "创建了用户", "target": "李四",
            "time": "刚刚", "variant": "success"
        }],
    });
    let html = engine().render("admin/dashboard.html", ctx).expect("dashboard.html 渲染失败");
    assert_x_data_well_formed(&html, "dashboard.html");
}

#[test]
fn 用户管理模板应可渲染() {
    let mut ctx = admin_ctx("用户管理");
    let obj = ctx.as_object_mut().unwrap();
    obj.insert("users".into(), json!([User::new("张三", "z@b.com", UserRole::Admin, UserStatus::Active, "2024-01-01")]));
    obj.insert("total".into(), json!(1));
    obj.insert("active_count".into(), json!(1));
    obj.insert("pending_count".into(), json!(0));
    obj.insert("inactive_count".into(), json!(0));

    let html = engine().render("admin/users.html", ctx).expect("users.html 渲染失败");
    assert_x_data_well_formed(&html, "users.html");
    // 新建用户弹窗应绑定 openCreate()，而非未定义的 openModal 赋值。
    assert!(html.contains("openCreate()"), "新建按钮应绑定 openCreate()");
    assert!(html.contains("x-model=\"form.name\""), "姓名输入应绑定 form.name");
    assert!(html.contains("x-model=\"form.email\""), "邮箱输入应绑定 form.email");
    assert!(html.contains("x-model=\"form.role\""), "角色选择应绑定 form.role");
    assert!(html.contains("create()"), "创建按钮应绑定 create()");
    // 分页应改为客户端驱动，不再出现服务端链接式分页。
    assert!(html.contains("goTo("), "分页应使用 goTo() 客户端切换");
    assert!(!html.contains("base_url"), "不应残留服务端分页参数");
    // 新外壳：中性顶栏 + 分组侧边栏 + 分段控件，且不再有浏览器式多页签栏。
    assert!(html.contains("class=\"topbar"), "应渲染中性顶栏");
    assert!(html.contains("menu-item"), "侧边栏应使用 menu-item 类");
    assert!(html.contains("menu-group-label"), "侧边栏应渲染分组标题");
    assert!(html.contains("tab-item"), "状态筛选应使用分段控件");
}

#[test]
fn 表单示例模板应可渲染() {
    let html = engine().render("admin/forms.html", admin_ctx("表单示例")).expect("forms.html 渲染失败");
    assert_x_data_well_formed(&html, "forms.html");
}

#[test]
fn 设置模板应可渲染() {
    let html = engine().render("admin/settings.html", admin_ctx("设置")).expect("settings.html 渲染失败");
    assert_x_data_well_formed(&html, "settings.html");
}

#[test]
fn 组件库模板应可渲染() {
    let mut ctx = admin_ctx("组件库");
    let obj = ctx.as_object_mut().unwrap();
    obj.insert(
        "components".into(),
        json!([{
            "category": "基础",
            "items": [{ "name": "button", "description": "按钮" }]
        }]),
    );
    obj.insert("chart".into(), chart_ctx());
    let html = engine().render("admin/ui-kit.html", ctx).expect("ui-kit.html 渲染失败");
    assert_x_data_well_formed(&html, "ui-kit.html");
}

#[test]
fn 登录页模板应可渲染() {
    engine()
        .render("admin/login.html", json!({ "title": "登录" }))
        .expect("login.html 渲染失败");
}

#[test]
fn 页面不存在模板应可渲染() {
    engine()
        .render("admin/404.html", json!({ "title": "页面不存在" }))
        .expect("404.html 渲染失败");
}

#[test]
fn 日历组件应可渲染() {
    let ctx = json!({
        "weeks": [{
            "days": [
                { "day": 27, "muted": false, "holiday": "中秋节",
                  "events": [{ "label": "白班", "time": "08:00-16:00", "variant": "blue" }] },
                { "day": 28, "muted": false, "holiday": "",
                  "events": [{ "label": "休息", "time": "", "variant": "green" }] },
            ]
        }]
    });
    let html = engine()
        .render_str(
            r#"{% import "components/calendar.html" as cal %}{{ cal.render(weeks=weeks) }}"#,
            ctx,
        )
        .expect("日历组件渲染失败");
    assert!(html.contains("周日"), "应渲染星期表头");
    assert!(html.contains("中秋节"), "应渲染节假日标记");
    assert!(html.contains("白班"), "应渲染事件条");
}

#[test]
fn 指标卡组应可渲染() {
    let ctx = json!({
        "items": [
            { "label": "总用户数", "value": "8", "variant": "blue", "icon": "<svg></svg>" },
            { "label": "活跃用户", "value": "5", "variant": "green", "icon": "<svg></svg>" },
        ]
    });
    let html = engine()
        .render_str(
            r#"{% import "components/data.html" as data %}{{ data.stat_group(items=items, cols=2) }}"#,
            ctx,
        )
        .expect("指标卡组渲染失败");
    assert!(html.contains("stat-card"), "应使用 stat-card 类");
    assert!(html.contains("总用户数"), "应渲染指标名称");
    // 色彩只出现在图标色块上，卡片本身保持白底。
    assert!(html.contains("bg-stat-blue-bg"), "应渲染蓝色变体");
    assert!(html.contains("stat-card\">"), "卡片本体不应带配色类，配色只落在图标块上");
}

#[test]
fn 面积图组件应可渲染() {
    let html = engine()
        .render_str(
            r#"{% import "components/chart.html" as chart %}{{ chart.area(chart=chart_data) }}"#,
            json!({ "chart_data": chart_ctx() }),
        )
        .expect("面积图渲染失败");
    assert!(html.contains("viewBox=\"0 0 720 240\""), "应输出固定 viewBox");
    assert!(html.contains("chart-grid-line"), "应渲染网格线");
    assert!(html.contains("preserveAspectRatio=\"none\""), "应横向自适应拉伸");
    assert!(html.contains("non-scaling-stroke"), "线宽不应被拉伸");
    assert!(html.contains("本年"), "应渲染图例");
    // 文字必须放在 SVG 之外，避免被 preserveAspectRatio 横向压扁。
    assert!(html.contains("8万"), "Y 轴标签应以 HTML 呈现");
}

#[test]
fn 分段控件应可渲染为现代分段样式() {
    let html = engine()
        .render_str(
            r#"{% import "components/navigation.html" as nav %}{{ nav.tabs(name="statusFilter", items=[{"id": "all", "label": "全部"}]) }}"#,
            json!({}),
        )
        .expect("分段控件渲染失败");
    assert!(html.contains("class=\"segmented\""), "应使用分段控件容器");
    assert!(html.contains("tab-item"), "分段项应使用 tab-item 类");
    assert!(html.contains("statusFilter = 'all'"), "应绑定 Alpine 状态变量");
}

#[test]
fn 提示条应可渲染() {
    let html = engine()
        .render_str(
            r#"{% import "components/calendar.html" as cal %}{{ cal.notice(text="测试提示", variant="warning", expandable=true) }}"#,
            json!({}),
        )
        .expect("提示条渲染失败");
    assert!(html.contains("测试提示"));
    assert!(html.contains("展开"));
}
