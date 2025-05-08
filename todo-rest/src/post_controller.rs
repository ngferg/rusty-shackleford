use std::sync::Arc;

use axum::{Json, extract::State, response::IntoResponse};
use serde_json::json;
use todo_lib::{self as lib, Task};

pub async fn add_task(
    State(dao): State<Arc<lib::Dao>>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let json_value = payload.get("description");
    match json_value {
        Some(task_description) => {
            let task_description = task_description
                .as_str()
                .expect("should be able to parse to string");
            let task_added = dao.add_task(task_description);
            match task_added {
                Some(id) => (
                    axum::http::StatusCode::CREATED,
                    Json(json!(Task {
                        id: id,
                        description: task_description.to_string()
                    })),
                ),
                _ => (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Failed to add task"})),
                ),
            }
        }
        _ => (
            axum::http::StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"error": "field \"description\" required"})),
        ),
    }
}
