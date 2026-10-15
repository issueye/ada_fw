//! JSON API 处理器。
//!
//! 提供数据的增删改查，供 Alpine.js 通过 `fetch` 调用。

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;

use crate::error::{AppError, AppResult};
use crate::models::{CreateTodo, CreateUser, Todo, User};
use crate::state::AppState;

/// `GET /api/todos` —— 列出全部待办。
pub async fn list_todos(State(state): State<AppState>) -> Json<Vec<Todo>> {
    Json(state.store.read().list())
}

/// `POST /api/todos` —— 新增待办。
pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodo>,
) -> AppResult<(StatusCode, Json<Todo>)> {
    let payload = payload.validate().map_err(AppError::BadRequest)?;
    let todo = state.store.write().add(payload.title);
    Ok((StatusCode::CREATED, Json(todo)))
}

/// `DELETE /api/todos/:id` —— 删除待办。
pub async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<StatusCode> {
    if state.store.write().remove(&id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}

/// `PATCH /api/todos/:id/toggle` —— 切换完成状态。
pub async fn toggle_todo(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<Todo>> {
    state
        .store
        .write()
        .toggle(&id)
        .map(Json)
        .ok_or(AppError::NotFound)
}

// ==========================================================================
// 用户
// ==========================================================================

/// `GET /api/users` —— 列出全部用户。
pub async fn list_users(State(state): State<AppState>) -> Json<Vec<User>> {
    Json(state.store.read().list_users())
}

/// `POST /api/users` —— 新增用户。
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> AppResult<(StatusCode, Json<User>)> {
    let payload = payload.validate().map_err(AppError::BadRequest)?;
    let user = state.store.write().add_user(payload.name, payload.email, payload.role);
    Ok((StatusCode::CREATED, Json(user)))
}

/// `DELETE /api/users/:id` —— 删除用户。
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<StatusCode> {
    if state.store.write().remove_user(&id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}
