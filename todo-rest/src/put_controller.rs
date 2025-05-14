use crate::lib;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::{Value, json};
use std::sync::Arc;

pub async fn update_task(
    Path(id): Path<String>,
    State(dao): State<Arc<lib::Dao>>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let id: i64 = id.parse().unwrap_or(0);
    if id <= 0 {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"error": "Supplied id that isn't a positive number"})),
        );
    }
    let action_opt = payload.get("action");
    match action_opt {
        Some(action) => {
            let action = action.as_str().unwrap_or("");
            match action.to_lowercase().as_str() {
                "finish" => finish_task(id, &dao),
                "unfinish" => unfinish_task(id, &dao),
                "update-description" => update_description(id, payload, &dao),
                _ => bad_action(),
            }
        }
        _ => bad_action(),
    }
}

fn update_description(id: i64, payload: Value, dao: &lib::Dao) -> (StatusCode, Json<Value>) {
    let new_description = payload.get("description");
    match new_description { 
        Some(new_description) => {
            let updated = dao.update_description(id, new_description.as_str().unwrap());
            match updated {
                true => (StatusCode::OK, Json(json!({"msg": "Task description updated"}))),
                false => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Failed to update task"})),
                ),
            }
        },
        _ => (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"error": "Field \"description\", is required to update description"})),
        )
    }
}

fn bad_action() -> (StatusCode, Json<Value>) {
    (
        StatusCode::UNPROCESSABLE_ENTITY,
        Json(json!({"error": "Not a valid \"action\", try finish or unfinish"})),
    )
}

fn finish_task(id: i64, dao: &lib::Dao) -> (StatusCode, Json<Value>) {
    let updated = dao.finish_task(id);
    match updated {
        true => (StatusCode::OK, Json(json!({"msg": "Task finished"}))),
        false => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to update task"})),
        ),
    }
}

fn unfinish_task(id: i64, dao: &lib::Dao) -> (StatusCode, Json<Value>) {
    let updated = dao.unfinish_task(id);
    match updated {
        true => (StatusCode::OK, Json(json!({"msg": "Task unfinished"}))),
        false => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to update task"})),
        ),
    }
}
