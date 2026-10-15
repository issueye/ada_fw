//! 应用共享状态。
//!
//! [`AppState`] 通过 `axum::extract::State` 注入到所有 handler，
//! 内部字段均为可廉价克隆的共享句柄。

use std::sync::Arc;

use parking_lot::RwLock;

use crate::config::AppConfig;
use crate::models::{Todo, User, UserRole, UserStatus};
use crate::template::TemplateEngine;

/// 内存数据存储（示例用，生产环境可替换为数据库连接池）。
#[derive(Debug, Default)]
pub struct Store {
    todos: Vec<Todo>,
    users: Vec<User>,
}

impl Store {
    /// 创建带示例数据的存储。
    pub fn with_seed() -> Self {
        Self {
            todos: vec![
                Todo::new("阅读 Ada Framework 架构文档"),
                Todo::new("体验 SSR 首屏渲染"),
                Todo::new("尝试 Alpine.js 局部更新"),
            ],
            users: vec![
                User::new("张伟", "zhangwei@example.com", UserRole::Admin, UserStatus::Active, "2024-01-15"),
                User::new("李娜", "lina@example.com", UserRole::Editor, UserStatus::Active, "2024-02-03"),
                User::new("王强", "wangqiang@example.com", UserRole::Viewer, UserStatus::Pending, "2024-02-18"),
                User::new("刘洋", "liuyang@example.com", UserRole::Editor, UserStatus::Inactive, "2024-03-07"),
                User::new("陈静", "chenjing@example.com", UserRole::Viewer, UserStatus::Active, "2024-03-22"),
                User::new("赵敏", "zhaomin@example.com", UserRole::Admin, UserStatus::Active, "2024-04-11"),
                User::new("孙磊", "sunlei@example.com", UserRole::Viewer, UserStatus::Pending, "2024-04-29"),
                User::new("周芳", "zhoufang@example.com", UserRole::Editor, UserStatus::Active, "2024-05-16"),
            ],
        }
    }

    // ---- 用户 ----

    /// 列出全部用户。
    pub fn list_users(&self) -> Vec<User> {
        self.users.clone()
    }

    /// 统计用户总数。
    pub fn user_count(&self) -> usize {
        self.users.len()
    }

    /// 按状态统计用户数。
    pub fn user_count_by_status(&self, status: UserStatus) -> usize {
        self.users.iter().filter(|u| u.status == status).count()
    }

    /// 新增用户。
    pub fn add_user(&mut self, name: impl Into<String>, email: impl Into<String>, role: UserRole) -> User {
        let user = User::new(name, email, role, UserStatus::Active, "2024-06-01");
        self.users.push(user.clone());
        user
    }

    /// 按 id 删除用户。
    pub fn remove_user(&mut self, id: &str) -> bool {
        let before = self.users.len();
        self.users.retain(|u| u.id != id);
        self.users.len() != before
    }

    /// 列出全部待办。
    pub fn list(&self) -> Vec<Todo> {
        self.todos.clone()
    }

    /// 新增一条待办，返回其副本。
    pub fn add(&mut self, title: impl Into<String>) -> Todo {
        let todo = Todo::new(title);
        self.todos.push(todo.clone());
        todo
    }

    /// 按 id 删除，返回是否删除成功。
    pub fn remove(&mut self, id: &str) -> bool {
        let before = self.todos.len();
        self.todos.retain(|t| t.id != id);
        self.todos.len() != before
    }

    /// 切换完成状态，返回更新后的待办。
    pub fn toggle(&mut self, id: &str) -> Option<Todo> {
        let todo = self.todos.iter_mut().find(|t| t.id == id)?;
        todo.done = !todo.done;
        Some(todo.clone())
    }
}

/// 应用全局状态。
#[derive(Clone)]
pub struct AppState {
    /// 配置。
    pub config: Arc<AppConfig>,
    /// 模板引擎。
    pub engine: TemplateEngine,
    /// 数据存储。
    pub store: Arc<RwLock<Store>>,
}

impl AppState {
    /// 构建应用状态。
    pub fn new(config: AppConfig, engine: TemplateEngine) -> Self {
        Self {
            config: Arc::new(config),
            engine,
            store: Arc::new(RwLock::new(Store::with_seed())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 新增待办后列表长度应增加() {
        let mut store = Store::default();
        assert_eq!(store.list().len(), 0);
        store.add("测试");
        assert_eq!(store.list().len(), 1);
    }

    #[test]
    fn 删除存在的待办应返回_true() {
        let mut store = Store::default();
        let todo = store.add("待删除");
        assert!(store.remove(&todo.id));
        assert!(store.list().is_empty());
    }

    #[test]
    fn 删除不存在的待办应返回_false() {
        let mut store = Store::default();
        assert!(!store.remove("not-exist"));
    }

    #[test]
    fn 切换状态应反转_done() {
        let mut store = Store::default();
        let todo = store.add("切换");
        let updated = store.toggle(&todo.id).unwrap();
        assert!(updated.done);
        let updated = store.toggle(&todo.id).unwrap();
        assert!(!updated.done);
    }

    #[test]
    fn 种子数据应包含_8_个用户() {
        let store = Store::with_seed();
        assert_eq!(store.user_count(), 8);
    }

    #[test]
    fn 新增用户后总数应增加() {
        let mut store = Store::default();
        assert_eq!(store.user_count(), 0);
        store.add_user("测试", "t@b.com", UserRole::Viewer);
        assert_eq!(store.user_count(), 1);
    }

    #[test]
    fn 删除存在的用户应返回_true() {
        let mut store = Store::default();
        let u = store.add_user("待删", "d@b.com", UserRole::Viewer);
        assert!(store.remove_user(&u.id));
        assert_eq!(store.user_count(), 0);
    }

    #[test]
    fn 删除不存在的用户应返回_false() {
        let mut store = Store::default();
        assert!(!store.remove_user("not-exist"));
    }

    #[test]
    fn 按状态统计用户数应正确() {
        let store = Store::with_seed();
        let active = store.user_count_by_status(UserStatus::Active);
        let pending = store.user_count_by_status(UserStatus::Pending);
        let inactive = store.user_count_by_status(UserStatus::Inactive);
        assert_eq!(active + pending + inactive, 8);
        assert_eq!(active, 5);
        assert_eq!(pending, 2);
        assert_eq!(inactive, 1);
    }
}
