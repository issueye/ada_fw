//! 领域模型模块。

pub mod todo;
pub mod user;

pub use todo::{CreateTodo, Todo};
pub use user::{CreateUser, User, UserRole, UserStatus};
