use std::collections::HashMap;

use axum::{Json, Router, extract::Query, routing::get};
use serde_json::{Value, json};
use todo_lib::{self as lib, QueryTodo};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route(
        "/tasks",
        get(get_tasks)
            .post(add_task)
            .put(update_task)
            .delete(delete_task),
    );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_tasks(Query(params): Query<HashMap<String, String>>) -> Json<Value> {
    let complete_tasks_only = params
        .get("complete_tasks_only")
        .unwrap_or(&"false".to_string())
        .parse()
        .unwrap_or(false);

    let home_dir = std::env::var("HOME").expect("HOME environment variable not set");
    let dao = lib::Dao::new(format!("{home_dir}/.ftodo/").as_str());

    Json(json!(dao.get_tasks(QueryTodo {
        incomplete_tasks_only: !complete_tasks_only
    })))
}

async fn add_task() {
    todo!()
}

async fn update_task() {
    todo!()
}

async fn delete_task() {
    todo!()
}
