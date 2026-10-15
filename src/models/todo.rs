//! 示例领域模型：待办事项。

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 待办事项。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    /// 唯一标识。
    pub id: String,
    /// 标题。
    pub title: String,
    /// 是否已完成。
    pub done: bool,
}

impl Todo {
    /// 创建一条新的待办事项。
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title: title.into(),
            done: false,
        }
    }

    /// 从持久化数据恢复一条待办（保留原 id 与完成状态）。
    pub fn restore(id: impl Into<String>, title: impl Into<String>, done: bool) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            done,
        }
    }
}

/// 创建待办的请求体。
#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    /// 标题，不能为空。
    pub title: String,
}

impl CreateTodo {
    /// 校验并归一化输入。
    pub fn validate(self) -> Result<Self, String> {
        let title = self.title.trim().to_string();
        if title.is_empty() {
            return Err("标题不能为空".to_string());
        }
        if title.chars().count() > 200 {
            return Err("标题长度不能超过 200 个字符".to_string());
        }
        Ok(Self { title })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 新建待办应默认未完成且带唯一_id() {
        let a = Todo::new("写文档");
        let b = Todo::new("写文档");
        assert!(!a.done);
        assert_ne!(a.id, b.id, "两条待办的 id 应不同");
    }

    #[test]
    fn 标题为空时应校验失败() {
        let req = CreateTodo {
            title: "   ".to_string(),
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn 标题应被去除首尾空白() {
        let req = CreateTodo {
            title: "  买菜  ".to_string(),
        };
        let ok = req.validate().unwrap();
        assert_eq!(ok.title, "买菜");
    }

    #[test]
    fn 超长标题应校验失败() {
        let req = CreateTodo {
            title: "a".repeat(201),
        };
        assert!(req.validate().is_err());
    }
}
