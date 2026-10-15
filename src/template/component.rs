//! 组件系统。
//!
//! 组件通过 Jinja 宏（macro）实现，约定：
//! - 每个组件一个 `.html` 文件，放在 `templates/components/`
//! - 组件内部用 `{% macro render(...) %}` 定义
//! - 页面通过 `{% import "components/xxx.html" as xxx %}` 引入
//!
//! **重要**：minijinja 的宏调用使用点号语法 `module.macro(...)`，
//! 不支持 Python Jinja2 的 `::` 命名空间语法。

use serde::Serialize;

/// 组件元信息，用于文档生成或调试。
#[derive(Debug, Clone, Serialize)]
pub struct ComponentMeta {
    /// 组件名（对应模板文件名，不含扩展名）。
    pub name: &'static str,
    /// 组件模板路径（相对 `templates/`）。
    pub path: &'static str,
    /// 组件分类。
    pub category: &'static str,
    /// 组件用途说明。
    pub description: &'static str,
}

/// 内置组件清单。
///
/// 新增组件时在此登记，便于统一管理与文档化。
pub const BUILTIN_COMPONENTS: &[ComponentMeta] = &[
    // ---- 基础组件 ----
    ComponentMeta {
        name: "button",
        path: "components/button.html",
        category: "基础",
        description: "按钮，支持 primary/secondary/outline/ghost/danger 变体与三档尺寸",
    },
    ComponentMeta {
        name: "input",
        path: "components/input.html",
        category: "基础",
        description: "输入框、多行文本、下拉选择，支持前后缀图标与错误态",
    },
    ComponentMeta {
        name: "form-controls",
        path: "components/form-controls.html",
        category: "基础",
        description: "复选框、单选框、开关",
    },
    // ---- 展示组件 ----
    ComponentMeta {
        name: "display",
        path: "components/display.html",
        category: "展示",
        description: "卡片、徽章、标签、头像、提示条、进度条、骨架屏、空状态",
    },
    // ---- 导航组件 ----
    ComponentMeta {
        name: "navigation",
        path: "components/navigation.html",
        category: "导航",
        description: "面包屑、视图切换、分页、下拉菜单、分组侧边栏、快捷导航面板",
    },
    ComponentMeta {
        name: "page",
        path: "components/page.html",
        category: "导航",
        description: "页面头与区块标题，统一各页的标题区结构",
    },
    // ---- 反馈组件 ----
    ComponentMeta {
        name: "feedback",
        path: "components/feedback.html",
        category: "反馈",
        description: "弹窗、抽屉、轻提示、文字提示、二次确认",
    },
    // ---- 数据组件 ----
    ComponentMeta {
        name: "data",
        path: "components/data.html",
        category: "数据",
        description: "数据表格、表单容器、指标卡、指标卡组",
    },
    ComponentMeta {
        name: "chart",
        path: "components/chart.html",
        category: "数据",
        description: "面积折线图、迷你趋势线、横向条形排行（纯 SVG，无第三方依赖）",
    },
    // ---- 业务组件 ----
    ComponentMeta {
        name: "calendar",
        path: "components/calendar.html",
        category: "业务",
        description: "排班日历、日历工具栏、提示条",
    },
];

/// 按名称查找组件元信息。
pub fn find_component(name: &str) -> Option<&'static ComponentMeta> {
    BUILTIN_COMPONENTS.iter().find(|c| c.name == name)
}

/// 按分类分组返回组件清单。
pub fn components_by_category() -> Vec<(&'static str, Vec<&'static ComponentMeta>)> {
    let mut categories: Vec<&'static str> = Vec::new();
    for c in BUILTIN_COMPONENTS {
        if !categories.contains(&c.category) {
            categories.push(c.category);
        }
    }
    categories
        .into_iter()
        .map(|cat| {
            let items = BUILTIN_COMPONENTS
                .iter()
                .filter(|c| c.category == cat)
                .collect();
            (cat, items)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 应能按名称找到内置组件() {
        let btn = find_component("button").expect("button 组件应存在");
        assert_eq!(btn.path, "components/button.html");
        assert_eq!(btn.category, "基础");
    }

    #[test]
    fn 查找不存在的组件应返回_none() {
        assert!(find_component("not_exist").is_none());
    }

    #[test]
    fn 组件清单应覆盖六个分类() {
        let cats = components_by_category();
        assert_eq!(cats.len(), 6, "应有 基础/展示/导航/反馈/数据/业务 六个分类");
    }

    #[test]
    fn 每个分类下都应有组件() {
        for (cat, items) in components_by_category() {
            assert!(!items.is_empty(), "分类 {cat} 下不应为空");
        }
    }
}
