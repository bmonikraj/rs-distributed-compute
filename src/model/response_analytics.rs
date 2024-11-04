use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseAnalytics {
    pub name: String,
    pub calls: i128,
    pub average_execution_duration_microseconds: f64,
}
