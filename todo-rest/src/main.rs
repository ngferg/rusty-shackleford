mod get_controller;
mod post_controller;
use std::sync::Arc;

use axum::{Router, routing::get};
use todo_lib::{self as lib};

#[tokio::main]
async fn main() {
    let home_dir = std::env::var("HOME").expect("HOME environment variable not set");
    let dao = Arc::new(lib::Dao::new(format!("{home_dir}/.ftodo/").as_str()));

    let app = Router::new()
        .route(
            "/tasks",
            get(get_controller::get_tasks)
                .post(post_controller::add_task)
                .put(update_task)
                .delete(delete_task),
        )
        .with_state(dao);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn update_task() {
    todo!()
}

async fn delete_task() {
    todo!()
}
