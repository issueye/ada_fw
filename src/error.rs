//! 统一错误类型与 HTTP 响应转换。

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

/// 应用统一错误类型。
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// 模板渲染失败。
    #[error("模板渲染失败: {0}")]
    Template(#[from] minijinja::Error),

    /// 资源不存在。
    #[error("资源不存在")]
    NotFound,

    /// 请求参数非法。
    #[error("请求参数非法: {0}")]
    BadRequest(String),

    /// 服务端内部错误。
    #[error("服务端内部错误: {0}")]
    Internal(String),
}

impl AppError {
    /// 对应的 HTTP 状态码。
    pub fn status(&self) -> StatusCode {
        match self {
            AppError::Template(_) | AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status();
        let message = self.to_string();

        // 服务端错误记录日志，避免细节泄露给客户端。
        if status.is_server_error() {
            tracing::error!(error = %message, "请求处理失败");
        }

        let body = Json(json!({
            "error": {
                "code": status.as_u16(),
                "message": message,
            }
        }));

        (status, body).into_response()
    }
}

/// 便捷结果别名。
pub type AppResult<T> = Result<T, AppError>;
