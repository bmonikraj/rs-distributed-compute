use std::collections::HashMap;
use actix_web::{web, App, HttpServer, Responder, Result};

use crate::model::health_info::{self, TraitInfoResponse};

async fn health_info() -> Result<impl Responder> {
    let mut response = health_info::InfoResponse::new();
    response.set("up!".to_string());
    Ok(web::Json(response))
}

#[actix_web::main]
pub async fn main(config: &HashMap<String, HashMap<String, String>>) -> std::io::Result<()> {
    let host: String = config["server"]["host"].to_string();
    let port: u16 = config["server"]["port"].to_string().parse().unwrap();
    let num_workers: usize = config["server"]["worker"].to_string().parse().unwrap();
    HttpServer::new(|| {
        App::new().service(
            web::scope("/health")
                .route("/status", web::get().to(health_info)),   
        )
    })
    .bind((host, port))?
    .workers(num_workers)
    .run()
    .await
}