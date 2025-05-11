use crate::lib;
use std::sync::Arc;
use axum::extract::{Path, State};
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;

pub async fn finish_task(Path(id): Path<String>,     State(dao): State<Arc<lib::Dao>>) -> impl IntoResponse {
    let id: i64 = id.parse().unwrap_or(0);
    if id == 0 {
        (
            axum::http::StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"error": "Supplied id that isn't a positive number"})),
        )
    } else {
        let updated = dao.finish_task(id);
        match updated {
            true =>         (
                axum::http::StatusCode::OK,
                Json(json!({"msg": "Task finished"})),
            ),
            false =>         (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to add task"})),
            )
        }
    }
}