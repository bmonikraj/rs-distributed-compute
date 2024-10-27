use std::{collections::HashMap, time::{SystemTime, UNIX_EPOCH}};

use axum::{http::StatusCode, response::{IntoResponse, Json}, routing::{get, post}, Router};
use serde_json::json;
use tokio::signal;

use crate::{model::request_compute::RequestCompute, service::{self, health::Health}};

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
    match axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await {
        Err(e)=> log::error!("error starting the server: {}", e),
        _ => (),
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

async fn handler_404() -> impl IntoResponse {
    return (StatusCode::NOT_FOUND, Json(json!({"message": "page not found!"})))
}

async fn handler_health_get() -> impl IntoResponse {
    log::info!("<health_get> handler in controller invoked");
    let health_service = Health::new();
    return (StatusCode::OK, Json(json!(health_service.info())));
}

async fn handler_compute_post(Json(request): Json<RequestCompute>) -> impl IntoResponse {
    log::info!("<compute_post> handler in controller invoked");
    let mut algorithm_service = match service::factory_algorithm::get_algorithm(request.name) {
        Ok(a) => a,
        Err(e) => return (StatusCode::NOT_IMPLEMENTED, Json(json!({"error": e.to_string()}))),
    };

    let timer = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros();
    let mut response = match algorithm_service.compute(request.param) {
        Ok(r) => r,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    };
    response.duration_micro_secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros() - timer;

    return (StatusCode::OK, Json(json!(response)));
}
