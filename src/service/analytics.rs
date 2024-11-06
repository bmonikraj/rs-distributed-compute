use core::f32;
use std::{error::Error, ptr::addr_of};

use sea_orm::{prelude::{Decimal}, sqlx::types::chrono::Utc, ActiveModelTrait, ConnectionTrait, DatabaseConnection, Set, Statement};

use crate::model::{entity_analytics, request_analytics::RequestAnalytics, response_analytics::ResponseAnalytics};

pub struct Analytics;

impl Analytics {
    pub fn new() -> Self {
        return Self {};
    }

    pub async fn insert(&self, db_client: &DatabaseConnection, algorithm: &String, parameter: &String) -> Result<entity_analytics::Model, Box<dyn Error>> {
        log::debug!("analytics::insert being invoked");
        let object = entity_analytics::ActiveModel {
            id: Set(uuid::Uuid::new_v4().to_owned()),
            algorithm: Set(algorithm.to_owned()),
            parameter: Set(parameter.to_owned()),
            result: Set("".to_owned()),
            created_at: Set(Utc::now().naive_utc().to_owned()),
            updated_at: Set(Utc::now().naive_utc().to_owned()),
        };
        log::debug!("address of db_client {:#?}", addr_of!(db_client));
        match object.insert(db_client).await {
            Ok(o) => return Ok(o),
            Err(e) => return Err(format!("error: {}", e.to_string()).into()),
        };
    }

    pub async fn update(&self, db_client: &DatabaseConnection, object: entity_analytics::Model, result: &String) -> Result<entity_analytics::Model, Box<dyn Error>> {
        log::debug!("analytics::update being invoked");
        let mut object: entity_analytics::ActiveModel = object.into();
        object.updated_at = Set(Utc::now().naive_utc().to_owned());
        object.result = Set(result.to_owned());
        log::debug!("address of db_client {:#?}", addr_of!(db_client));
        match object.update(db_client).await {
            Ok(o) => return Ok(o),
            Err(e) => return Err(format!("error: {}", e.to_string()).into()),
        };
    }

    pub async fn analyse(&self, db_client: &DatabaseConnection, request: &RequestAnalytics) -> Result<ResponseAnalytics, Box<dyn Error>> {        
        log::debug!("analytics::analyse being invoked");
        let mut response = ResponseAnalytics {
            name: request.name.clone(),
            calls: 5,
            average_execution_duration_seconds: 10.0,
        };
        log::debug!("address of db_client {:#?}", addr_of!(db_client));
        let query_result_option = match db_client.query_one(
            Statement::from_string(
                sea_orm::DatabaseBackend::Postgres, 
                format!("select extract('epoch' from AVG(analytics.updated_at - analytics.created_at)) as avg_, count(*) as call_ from public.analytics where analytics.algorithm like '{}';", request.name.clone()),
            )
        ).await {
            Ok(r) => r,
            Err(e) => return Err(format!("error: {}", e.to_string()).into()),
        };

        let query_result = match query_result_option {
            Some(q) => q,
            None => return Err(format!("error: no result retrieved from query").into()),
        };

        let call_: i64 = match query_result.try_get("", "call_") {
            Ok(c) => c,
            Err(e) => return Err(format!("error: {}", e.to_string()).into()),
        };

        let avg_exec_time_secs_: Decimal = match query_result.try_get("", "avg_") {
            Ok(c) => c,
            Err(e) => return Err(format!("error: {}", e.to_string()).into()),
        };

        response.calls = call_;
        response.average_execution_duration_seconds = match avg_exec_time_secs_.to_string().parse::<f32>() {
            Ok(f) => f,
            Err(e) => return Err(format!("error: {}", e.to_string()).into()),
        };

        Ok(response)
    }
}