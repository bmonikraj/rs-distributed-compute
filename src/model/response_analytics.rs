use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseAnalytics {
    pub name: String,
    pub calls: i64,
    pub average_execution_duration_seconds: f32,
}
