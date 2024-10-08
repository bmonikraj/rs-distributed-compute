use std::collections::HashMap;

use axum::{http::StatusCode, response::{IntoResponse, Json}, routing::{get, post}, Router};
use serde_json::json;

use crate::service::{health::Health, t_health::THealth};

#[tokio::main]
pub async fn start(config: &HashMap<String, HashMap<String, String>>) {
    let app = Router::new()
        .route("/health", get(handler_health_get))
        .route("/task", post(handler_task_post).get(handler_task_get_all))
        .route("/task/:id", get(handler_task_get).delete(handler_task_delete).patch(handler_task_update));

    let app = app.fallback(handler_404);

    let host = config["server"]["host"].to_string();
    let port = config["server"]["port"].to_string();
    let listener = match tokio::net::TcpListener::bind(format!("{host}:{port}"))
        .await {
            Ok(l) => l,
            Err(_) => tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap(),
        };

    log::info!("starting server on {:#?}", listener.local_addr().unwrap());
    match axum::serve(listener, app).await {
        Err(e)=> log::error!("error starting the server: {}", e),
        _ => (),
    }
}

async fn handler_404() -> impl IntoResponse {
    return (StatusCode::NOT_FOUND, Json(json!({"message": "page not found!"})))
}

async fn handler_health_get() -> impl IntoResponse {
    log::info!("<health_get> handler in controller invoked");
    let health_service = Health::new();
    return (StatusCode::OK, Json(health_service.info()));
}

async fn handler_task_get_all() -> () {
    log::info!("<task_get_all> handler in controller invoked");
    todo!()
    // get all data from db through manager
}

async fn handler_task_get() -> () {
    log::info!("<task_get> handler in controller invoked");
    todo!()
    // get specific data from db through manager
}

async fn handler_task_post() -> () {
    log::info!("<task_post> handler in controller invoked");
    todo!()
    // create task in db through manager and ask worker to start the work - ml train and publish model in minio 
}

async fn handler_task_update() -> () {
    log::info!("<task_update> handler in controller invoked");
    todo!()
    // update meta data in db through manager
}

async fn handler_task_delete() -> () {
    log::info!("<task_delete> handler in controller invoked");
    todo!()
    // delete the minio model through worker and data in db through manager
}