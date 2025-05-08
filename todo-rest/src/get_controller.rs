use std::{collections::HashMap, sync::Arc};

use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};
use serde_json::json;
use todo_lib::{self as lib, QueryTodo};

pub async fn get_tasks(
    State(dao): State<Arc<lib::Dao>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let complete_tasks_only = params
        .get("complete_tasks_only")
        .unwrap_or(&"false".to_string())
        .parse()
        .unwrap_or(false);

    Json(json!(dao.get_tasks(QueryTodo {
        incomplete_tasks_only: !complete_tasks_only
    })))
}
