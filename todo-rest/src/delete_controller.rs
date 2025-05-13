use crate::lib;
use axum::Json;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use serde_json::json;
use std::sync::Arc;

pub async fn delete_task(
    Path(id): Path<String>,
    State(dao): State<Arc<lib::Dao>>,
) -> impl IntoResponse {
    let id: i64 = id.parse().unwrap_or(0);
    if id == 0 {
        (
            axum::http::StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"error": "Supplied id that isn't a positive number"})),
        )
    } else {
        let deleted = dao.delete_task(id);
        match deleted {
            true => (
                axum::http::StatusCode::OK,
                Json(json!({"msg": "Task deleted"})),
            ),
            false => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to delete task"})),
            ),
        }
    }
}

pub async fn delete_all_tasks(State(dao): State<Arc<lib::Dao>>) -> impl IntoResponse {
    let deleted = dao.reset_db();
    match deleted {
        true => (
            axum::http::StatusCode::OK,
            Json(json!({"msg": "All tasks deleted"})),
        ),
        false => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to delete task"})),
        ),
    }
}
