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

/// 构造一个完整的后台页面上下文。
fn admin_ctx(title: &str) -> serde_json::Value {
    json!({
        "title": title,
        "breadcrumb": [
            { "label": "首页", "href": "/admin" },
            { "label": title, "href": "" }
        ],
        "menu_items": [
            { "label": "仪表盘", "href": "/admin", "icon": "<svg></svg>", "active": true }
        ],
        "tabs": [
            { "label": "首页", "href": "/admin", "active": true, "closable": false },
            { "label": "用户管理", "href": "/admin/users", "active": false, "closable": true }
        ],
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
        "title": "仪表盘",
        "breadcrumb": [{ "label": "仪表盘", "href": "" }],
        "menu_items": [],
        "tabs": [{ "label": "首页", "href": "/admin", "active": true, "closable": false }],
        "stats": [{
            "label": "总用户数", "value": "8", "trend": "+1%",
            "trend_up": true, "icon": "<svg></svg>", "variant": "primary"
        }],
        "recent_users": [User::new("张三", "z@b.com", UserRole::Admin, UserStatus::Active, "2024-01-01")],
        "activities": [{
            "user": "张三", "action": "创建了用户", "target": "李四",
            "time": "刚刚", "variant": "success"
        }],
    });
    engine().render("admin/dashboard.html", ctx).expect("dashboard.html 渲染失败");
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
    // 阶段一修复点：新建用户弹窗应绑定 openCreate()，而非未定义的 openModal 赋值。
    assert!(html.contains("openCreate()"), "新建按钮应绑定 openCreate()");
    assert!(html.contains("x-model=\"form.name\""), "姓名输入应绑定 form.name");
    assert!(html.contains("x-model=\"form.email\""), "邮箱输入应绑定 form.email");
    assert!(html.contains("x-model=\"form.role\""), "角色选择应绑定 form.role");
    assert!(html.contains("create()"), "创建按钮应绑定 create()");
    // 分页应改为客户端驱动，不再出现服务端链接式分页。
    assert!(html.contains("goTo("), "分页应使用 goTo() 客户端切换");
    assert!(!html.contains("base_url"), "不应残留服务端分页参数");
    // 新布局：蓝色顶栏 + 多页签栏。
    assert!(html.contains("class=\"topbar"), "应渲染蓝色顶栏");
    assert!(html.contains("tab-item"), "应渲染多页签栏");
    assert!(html.contains("menu-item"), "侧边栏应使用 menu-item 类");
}

#[test]
fn 表单示例模板应可渲染() {
    engine().render("admin/forms.html", admin_ctx("表单示例")).expect("forms.html 渲染失败");
}

#[test]
fn 设置模板应可渲染() {
    engine().render("admin/settings.html", admin_ctx("设置")).expect("settings.html 渲染失败");
}

#[test]
fn 组件库模板应可渲染() {
    let mut ctx = admin_ctx("组件库");
    ctx.as_object_mut().unwrap().insert(
        "components".into(),
        json!([{
            "category": "基础",
            "items": [{ "name": "button", "description": "按钮" }]
        }]),
    );
    engine().render("admin/ui-kit.html", ctx).expect("ui-kit.html 渲染失败");
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
            { "label": "总用户数", "value": "8", "variant": "blue" },
            { "label": "活跃用户", "value": "5", "variant": "green" },
        ]
    });
    let html = engine()
        .render_str(
            r#"{% import "components/data.html" as data %}{{ data.stat_group(items=items, cols=2) }}"#,
            ctx,
        )
        .expect("指标卡组渲染失败");
    assert!(html.contains("stat-card"), "���使用 stat-card 类");
    assert!(html.contains("总用户数"), "应渲染指标名称");
    assert!(html.contains("bg-stat-blue-bg"), "应渲染蓝色变体");
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
