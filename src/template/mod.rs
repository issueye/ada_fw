//! 模板引擎封装。
//!
//! 基于 [`minijinja`] 提供：
//! - 递归加载 `templates/` 目录下的所有模板
//! - 注册全局函数（如 `asset()`）
//! - 统一的渲染入口
//! - 开发模式下的模板热重载（监听目录变更并重建环境）

use std::path::{Path, PathBuf};
use std::sync::Arc;

use minijinja::{path_loader, Environment};
use parking_lot::RwLock;
use serde::Serialize;

use crate::config::AppConfig;
use crate::error::AppResult;

pub mod component;

/// 模板引擎。
///
/// 内部持有 `Arc<RwLock<Arc<Environment>>>`：
/// - 常规模式：环境构建一次后不再变更，读锁开销极低
/// - 热重载模式：文件变更时替换内部环境，渲染方读取最新快照
#[derive(Clone)]
pub struct TemplateEngine {
    env: Arc<RwLock<Arc<Environment<'static>>>>,
    /// 模板目录（热重载监听用）。
    template_dir: PathBuf,
}

impl TemplateEngine {
    /// 基于配置创建模板引擎。
    pub fn new(config: &AppConfig) -> AppResult<Self> {
        let env = build_env(config)?;

        Ok(Self {
            env: Arc::new(RwLock::new(Arc::new(env))),
            template_dir: config.template_dir.clone(),
        })
    }

    /// 启动模板热重载监听（仅开发模式调用）。
    ///
    /// 监听 `templates/` 目录，任意 `.html` 变更时重建模板环境。
    /// 监听线程为后台守护线程，随进程退出而结束。
    pub fn watch(&self, config: &AppConfig) -> AppResult<()> {
        use notify::{RecursiveMode, Watcher};

        let dir = self.template_dir.clone();
        let env_slot = Arc::clone(&self.env);
        let cfg = config.clone();

        let (tx, rx) = std::sync::mpsc::channel();

        let mut watcher = notify::recommended_watcher(move |res| {
            let _ = tx.send(res);
        })
        .map_err(|e| crate::error::AppError::Internal(format!("创建文件监听器失败: {e}")))?;

        watcher
            .watch(&dir, RecursiveMode::Recursive)
            .map_err(|e| crate::error::AppError::Internal(format!("监听模板目录失败: {e}")))?;

        std::thread::spawn(move || {
            // 持有 watcher，避免其被提前释放。
            let _watcher = watcher;
            for res in rx {
                match res {
                    Ok(event) if event.paths.iter().any(|p| is_template(p)) => {
                        match build_env(&cfg) {
                            Ok(new_env) => {
                                *env_slot.write() = Arc::new(new_env);
                                tracing::info!("模板已热重载");
                            }
                            Err(e) => {
                                // 保留旧环境，避免语法错误导致服务不可用。
                                tracing::error!(error = %e, "模板重载失败，继续使用旧版本");
                            }
                        }
                    }
                    Ok(_) => {}
                    Err(e) => tracing::warn!(error = %e, "模板监听事件异常"),
                }
            }
        });

        tracing::info!("模板热重载已启用: {}", self.template_dir.display());
        Ok(())
    }

    /// 渲染指定模板。
    ///
    /// `name` 为相对 `templates/` 的路径，如 `index.html` 或 `components/nav.html`。
    pub fn render<S: Serialize>(&self, name: &str, ctx: S) -> AppResult<String> {
        // 读取当前环境快照（克隆 Arc，释放锁后再渲染）。
        let env = Arc::clone(&self.env.read());
        let tmpl = env.get_template(name)?;
        let html = tmpl.render(ctx)?;
        Ok(html)
    }

    /// 直接渲染一段模板字符串（用于测试或动态片段）。
    pub fn render_str<S: Serialize>(&self, source: &str, ctx: S) -> AppResult<String> {
        let env = Arc::clone(&self.env.read());
        let html = env.render_str(source, ctx)?;
        Ok(html)
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

/// 构建模板环境（加载目录、注册全局函数与变量）。
fn build_env(config: &AppConfig) -> AppResult<Environment<'static>> {
    let mut env = Environment::new();

    // 递归加载模板目录。
    env.set_loader(path_loader(&config.template_dir));

    // 注册全局函数：生成静态资源路径。
    env.add_function("asset", |path: String| format!("/static/{path}"));

    // 注册全局变量：Tailwind 模式，供 base.html 条件渲染。
    env.add_global("tailwind_mode", config.tailwind_mode.as_str());

    // 开发模式下保留模板源信息，便于定位错误。
    env.set_debug(config.hot_reload);

    Ok(env)
}

/// 判断路径是否为模板文件。
fn is_template(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.eq_ignore_ascii_case("html"))
        .unwrap_or(false)
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

    #[test]
    fn 引擎克隆后应共享同一环境() {
        let engine = engine();
        let cloned = engine.clone();
        let out = cloned
            .render_str("{{ 1 + 1 }}", serde_json::json!({}))
            .unwrap();
        assert_eq!(out, "2");
    }

    #[test]
    fn 仅_html_文件应被识别为模板() {
        assert!(is_template(Path::new("templates/index.html")));
        assert!(is_template(Path::new("templates/INDEX.HTML")));
        assert!(!is_template(Path::new("static/app.css")));
        assert!(!is_template(Path::new("templates/noext")));
    }
}
