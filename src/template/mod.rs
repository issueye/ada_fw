//! 模板引擎封装。
//!
//! 基于 [`minijinja`] 提供：
//! - 递归加载 `templates/` 目录下的所有模板
//! - 注册全局函数（如 `asset()`）
//! - 统一的渲染入口

use std::path::Path;
use std::sync::Arc;

use minijinja::{path_loader, Environment};
use serde::Serialize;

use crate::config::AppConfig;
use crate::error::AppResult;

pub mod component;

/// 模板引擎，内部持有 `Arc`，可低成本克隆并跨线程共享。
#[derive(Clone)]
pub struct TemplateEngine {
    env: Arc<Environment<'static>>,
}

impl TemplateEngine {
    /// 基于配置创建模板引擎。
    pub fn new(config: &AppConfig) -> AppResult<Self> {
        let mut env = Environment::new();

        // 递归加载模板目录。
        env.set_loader(path_loader(&config.template_dir));

        // 注册全局函数：生成静态资源路径。
        env.add_function("asset", |path: String| format!("/static/{path}"));

        // 注册全局变量：Tailwind 模式，供 base.html 条件渲染。
        env.add_global("tailwind_mode", config.tailwind_mode.as_str());

        // 开发模式下保留模板源信息，便于定位错误。
        env.set_debug(config.hot_reload);

        Ok(Self {
            env: Arc::new(env),
        })
    }

    /// 渲染指定模板。
    ///
    /// `name` 为相对 `templates/` 的路径，如 `index.html` 或 `components/nav.html`。
    pub fn render<S: Serialize>(&self, name: &str, ctx: S) -> AppResult<String> {
        let tmpl = self.env.get_template(name)?;
        let html = tmpl.render(ctx)?;
        Ok(html)
    }

    /// 直接渲染一段模板字符串（用于测试或动态片段）。
    pub fn render_str<S: Serialize>(&self, source: &str, ctx: S) -> AppResult<String> {
        let html = self.env.render_str(source, ctx)?;
        Ok(html)
    }

    /// 暴露底层环境，供高级用法（如注册自定义过滤器）。
    pub fn env(&self) -> &Environment<'static> {
        &self.env
    }

    /// 检查模板目录是否存在，便于启动期快速失败。
    pub fn ensure_dir(dir: &Path) -> AppResult<()> {
        if !dir.exists() {
            return Err(crate::error::AppError::Internal(format!(
                "模板目录不存在: {}",
                dir.display()
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;

    fn engine() -> TemplateEngine {
        let cfg = AppConfig::default();
        TemplateEngine::new(&cfg).expect("引擎创建失败")
    }

    #[test]
    fn 渲染字符串模板应正确替换变量() {
        let engine = engine();
        let out = engine
            .render_str("你好, {{ name }}!", serde_json::json!({ "name": "Ada" }))
            .unwrap();
        assert_eq!(out, "你好, Ada!");
    }

    #[test]
    fn asset_函数应生成静态资源路径() {
        let engine = engine();
        let out = engine
            .render_str("{{ asset('css/app.css') }}", serde_json::json!({}))
            .unwrap();
        assert_eq!(out, "/static/css/app.css");
    }

    #[test]
    fn 渲染不存在的模板应返回错误() {
        let engine = engine();
        let result = engine.render("not_exist.html", serde_json::json!({}));
        assert!(result.is_err());
    }
}
