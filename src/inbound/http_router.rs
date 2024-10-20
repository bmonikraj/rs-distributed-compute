use std::collections::HashMap;

use axum::{http::StatusCode, response::{IntoResponse, Json}, routing::{get, post}, Router};
use serde_json::json;

use crate::service::{self, health::Health, t_algorithm::Algorithm};

#[tokio::main]
pub async fn start(config: &HashMap<String, HashMap<String, String>>) {
    let app = Router::new()
        .route("/health", get(handler_health_get))
        .route("/compute", post(handler_compute_post));
        
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

async fn handler_compute_post() -> () {
    log::info!("<compute_post> handler in controller invoked");
    let mut algorithm_service = service::factory_algorithm::get_algorithm("name".to_owned()).unwrap();
    // match algorithm_service.compute("param".to_string()) {
        
    // }
}
