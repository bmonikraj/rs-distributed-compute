use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseHealth {
    pub status: String
}
