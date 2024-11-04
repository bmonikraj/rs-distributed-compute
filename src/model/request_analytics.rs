use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestAnalytics {
    pub name: String,
}
