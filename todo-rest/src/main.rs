use std::{collections::HashMap, sync::Arc};

use axum::{
    Json, Router,
    extract::{Query, State},
    response::IntoResponse,
    routing::get,
};
use serde_json::json;
use todo_lib::{self as lib, QueryTodo, Task};

#[tokio::main]
async fn main() {
    let home_dir = std::env::var("HOME").expect("HOME environment variable not set");
    let dao = Arc::new(lib::Dao::new(format!("{home_dir}/.ftodo/").as_str()));

    let app = Router::new()
        .route(
            "/tasks",
            get(get_tasks)
                .post(add_task)
                .put(update_task)
                .delete(delete_task),
        )
        .with_state(dao);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_tasks(
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

async fn add_task(
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

async fn update_task() {
    todo!()
}

async fn delete_task() {
    todo!()
}
