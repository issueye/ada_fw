//! 示例领域模型：用户。

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 用户角色。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    /// 管理员。
    Admin,
    /// 编辑。
    Editor,
    /// 访客。
    Viewer,
}

impl UserRole {
    /// 中文显示名。
    pub fn label(&self) -> &'static str {
        match self {
            UserRole::Admin => "管理员",
            UserRole::Editor => "编辑",
            UserRole::Viewer => "访客",
        }
    }

    /// 对应的徽章变体。
    pub fn variant(&self) -> &'static str {
        match self {
            UserRole::Admin => "primary",
            UserRole::Editor => "info",
            UserRole::Viewer => "neutral",
        }
    }
}

/// 用户状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserStatus {
    /// 启用。
    Active,
    /// 停用。
    Inactive,
    /// 待激活。
    Pending,
}

impl UserStatus {
    /// 中文显示名。
    pub fn label(&self) -> &'static str {
        match self {
            UserStatus::Active => "启用",
            UserStatus::Inactive => "停用",
            UserStatus::Pending => "待激活",
        }
    }

    /// 对应的徽章变体。
    pub fn variant(&self) -> &'static str {
        match self {
            UserStatus::Active => "success",
            UserStatus::Inactive => "neutral",
            UserStatus::Pending => "warning",
        }
    }
}

/// 用户。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// 唯一标识。
    pub id: String,
    /// 姓名。
    pub name: String,
    /// 邮箱。
    pub email: String,
    /// 角色。
    pub role: UserRole,
    /// 状态。
    pub status: UserStatus,
    /// 创建日期（YYYY-MM-DD）。
    pub created_at: String,
    /// 角色中文名（序列化派生字段，供模板直接使用）。
    #[serde(rename = "role_label")]
    pub role_label: String,
    /// 角色徽章变体（序列化派生字段）。
    #[serde(rename = "role_variant")]
    pub role_variant: String,
    /// 状态中文名（序列化派生字段）。
    #[serde(rename = "status_label")]
    pub status_label: String,
    /// 状态徽章变体（序列化派生字段）。
    #[serde(rename = "status_variant")]
    pub status_variant: String,
    /// 姓名首字母（序列化派生字段，用于头像）。
    pub initial: String,
}

impl User {
    /// 创建用户。
    pub fn new(
        name: impl Into<String>,
        email: impl Into<String>,
        role: UserRole,
        status: UserStatus,
        created_at: impl Into<String>,
    ) -> Self {
        let name = name.into();
        let initial = name.chars().next().map(|c| c.to_string()).unwrap_or_default();

        Self {
            id: Uuid::new_v4().to_string(),
            name,
            email: email.into(),
            role,
            status,
            created_at: created_at.into(),
            role_label: role.label().to_string(),
            role_variant: role.variant().to_string(),
            status_label: status.label().to_string(),
            status_variant: status.variant().to_string(),
            initial,
        }
    }

    /// 从持久化数据恢复用户（保留原 id，并重建派生字段）。
    pub fn restore(
        id: impl Into<String>,
        name: impl Into<String>,
        email: impl Into<String>,
        role: UserRole,
        status: UserStatus,
        created_at: impl Into<String>,
    ) -> Self {
        let name = name.into();
        let initial = name.chars().next().map(|c| c.to_string()).unwrap_or_default();

        Self {
            id: id.into(),
            name,
            email: email.into(),
            role,
            status,
            created_at: created_at.into(),
            role_label: role.label().to_string(),
            role_variant: role.variant().to_string(),
            status_label: status.label().to_string(),
            status_variant: status.variant().to_string(),
            initial,
        }
    }
}

/// 创建用户的请求体。
#[derive(Debug, Deserialize)]
pub struct CreateUser {
    /// 姓名。
    pub name: String,
    /// 邮箱。
    pub email: String,
    /// 角色。
    #[serde(default = "default_role")]
    pub role: UserRole,
}

fn default_role() -> UserRole {
    UserRole::Viewer
}

impl CreateUser {
    /// 校验并归一化输入。
    pub fn validate(self) -> Result<Self, String> {
        let name = self.name.trim().to_string();
        let email = self.email.trim().to_string();

        if name.is_empty() {
            return Err("姓名不能为空".to_string());
        }
        if name.chars().count() > 50 {
            return Err("姓名长度不能超过 50 个字符".to_string());
        }
        if !email.contains('@') || email.len() < 3 {
            return Err("邮箱格式不正确".to_string());
        }
        if email.chars().count() > 100 {
            return Err("邮箱长度不能超过 100 个字符".to_string());
        }

        Ok(Self {
            name,
            email,
            role: self.role,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 新建用户应带唯一_id() {
        let a = User::new("张三", "a@b.com", UserRole::Admin, UserStatus::Active, "2024-01-01");
        let b = User::new("张三", "a@b.com", UserRole::Admin, UserStatus::Active, "2024-01-01");
        assert_ne!(a.id, b.id);
    }

    #[test]
    fn 角色与状态应返回正确的中文标签() {
        let u = User::new("李四", "l@b.com", UserRole::Editor, UserStatus::Pending, "2024-01-01");
        assert_eq!(u.role_label, "编辑");
        assert_eq!(u.status_label, "待激活");
        assert_eq!(u.role_variant, "info");
        assert_eq!(u.status_variant, "warning");
    }

    #[test]
    fn 姓名首字母应取第一个字符() {
        let u = User::new("王五", "w@b.com", UserRole::Viewer, UserStatus::Active, "2024-01-01");
        assert_eq!(u.initial, "王");
    }

    #[test]
    fn 序列化后应包含派生字段() {
        let u = User::new("赵六", "z@b.com", UserRole::Admin, UserStatus::Active, "2024-01-01");
        let json = serde_json::to_value(&u).unwrap();
        assert_eq!(json["role_label"], "管理员");
        assert_eq!(json["status_label"], "启用");
        assert_eq!(json["initial"], "赵");
    }

    #[test]
    fn 姓名为空时应校验失败() {
        let req = CreateUser {
            name: "  ".to_string(),
            email: "a@b.com".to_string(),
            role: UserRole::Viewer,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn 邮箱格式错误时应校验失败() {
        let req = CreateUser {
            name: "张三".to_string(),
            email: "invalid-email".to_string(),
            role: UserRole::Viewer,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn 合法输入应通过校验并去除空白() {
        let req = CreateUser {
            name: "  张三  ".to_string(),
            email: "  z@b.com  ".to_string(),
            role: UserRole::Admin,
        };
        let ok = req.validate().unwrap();
        assert_eq!(ok.name, "张三");
        assert_eq!(ok.email, "z@b.com");
    }
}
