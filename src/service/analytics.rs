use std::error::Error;

use sea_orm::{sqlx::types::chrono::Utc, ActiveModelTrait, DatabaseConnection, Set};

use crate::model::{entity_analytics, request_analytics::RequestAnalytics, response_analytics::ResponseAnalytics};

pub struct Analytics;

impl Analytics {
    pub fn new() -> Self {
        return Self {};
    }

    pub async fn insert(&self, db_client: &DatabaseConnection, algorithm: &String, parameter: &String) -> Result<entity_analytics::Model, Box<dyn Error>> {
        let object = entity_analytics::ActiveModel {
            id: Set(uuid::Uuid::new_v4().to_owned()),
            algorithm: Set(algorithm.to_owned()),
            parameter: Set(parameter.to_owned()),
            result: Set("".to_owned()),
            created_at: Set(Utc::now().naive_utc().to_owned()),
            updated_at: Set(Utc::now().naive_utc().to_owned()),
        };
        match object.insert(db_client).await {
            Ok(o) => return Ok(o),
            Err(e) => return Err(format!("error: {}", e.to_string()).into()),
        };
    }

    pub async fn update(&self, db_client: &DatabaseConnection, object: entity_analytics::Model, result: &String) -> Result<entity_analytics::Model, Box<dyn Error>> {
        let mut object: entity_analytics::ActiveModel = object.into();
        object.updated_at = Set(Utc::now().naive_utc().to_owned());
        object.result = Set(result.to_owned());
        match object.update(db_client).await {
            Ok(o) => return Ok(o),
            Err(e) => return Err(format!("error: {}", e.to_string()).into()),
        };
    }

    pub async fn analyse(&self, _db_client: &DatabaseConnection, request: &RequestAnalytics) -> Result<ResponseAnalytics, Box<dyn Error>> {        
        let response = ResponseAnalytics {
            name: request.name.clone(),
            calls: 5,
            average_execution_duration_microseconds: 10.0,
        };
        if request.name == "a" {
            return Ok(response);
        } else {
            return Err(format!("e_r_r_o_r").into());
        }
    }
}