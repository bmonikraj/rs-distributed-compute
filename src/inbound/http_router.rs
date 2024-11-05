use std::{collections::HashMap, error::Error, time::{SystemTime, UNIX_EPOCH}};

use axum::{extract::State, http::StatusCode, response::{IntoResponse, Json}, routing::{get, post}, Router};
use sea_orm::DatabaseConnection;
use serde_json::json;
use tokio::signal;

use crate::{model::{entity_analytics::Model, request_analytics::RequestAnalytics, request_compute::RequestCompute}, outbound::db_driver::DbDriver, service::{self, analytics::Analytics, health::Health}};

#[tokio::main]
pub async fn start(config: &HashMap<String, HashMap<String, String>>, db_driver: DbDriver) {
    
    let db_client = match futures::executor::block_on(db_driver.get_db()) {
        Ok(c) => c,
        Err(e) => panic!("db error: {}", e),
    };
    
    let app = Router::new()
        .route("/health", get(handler_health_get))
        .route("/compute", post(handler_compute_post))
        .route("/analytics", post(handler_analytics_post))
        .with_state(db_client);
        
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

// #[debug_handler]
async fn handler_compute_post(State(db_driver): State<DatabaseConnection>, Json(request): Json<RequestCompute>) -> impl IntoResponse {
    log::info!("<compute_post> handler in controller invoked");

    let mut algorithm_service = match service::factory_algorithm::get_algorithm(&request.name) {
        Ok(a) => a,
        Err(e) => return (StatusCode::NOT_IMPLEMENTED, Json(json!({"error": e.to_string()}))),
    };

    let db_model = match __compute_insert(&db_driver, &request.name, &request.param) {
        Ok(d) => d,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    };

    let timer = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros();
    let mut response = match algorithm_service.compute(&request.param) {
        Ok(r) => r,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    };
    response.duration_micro_secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros() - timer;

    match __compute_update(&db_driver, &db_model, &response.result) {
        Ok(_) => (),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    }

    return (StatusCode::OK, Json(json!(response)));
}

async fn handler_analytics_post(State(db_client): State<DatabaseConnection>, Json(request): Json<RequestAnalytics>) -> impl IntoResponse {  
    // let db_client = match db_driver.get_db().await {
    //     Ok(c) => c,
    //     Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    // };
    
    let analytics_service = Analytics::new();

    let response = match futures::executor::block_on(analytics_service.analyse(&db_client, &request)) {
        Ok(r) => r,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    };

    return (StatusCode::OK, Json(json!(response)));
}

fn __compute_insert(db_client: &DatabaseConnection, name: &String, param: &String) -> Result<Model, Box<dyn Error>> {
    log::debug!("__compute_insert being called");
    let analytics_service = Analytics::new();
    let _db_object = match futures::executor::block_on(analytics_service.insert(&db_client, name, param)) {
        Ok(d) => return Ok(d),
        Err(e) => return Err(format!("{}", e.to_string()).into()),
    };
}

fn __compute_update(db_client: &DatabaseConnection, db_object: &Model, result: &String) -> Result<Model, Box<dyn Error>> {
    log::debug!("__compute_update being called");
    let analytics_service = Analytics::new();
    match futures::executor::block_on(analytics_service.update(&db_client, db_object.clone(), result)) {
        Ok(d) => return Ok(d),
        Err(e) => return Err(format!("{}", e.to_string()).into()),
    }
}