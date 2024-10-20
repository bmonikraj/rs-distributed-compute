use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseCompute {
    pub name: String,
    pub result: String
}
