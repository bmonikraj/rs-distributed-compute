use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestCompute {
    pub name: String,
    pub param: String
}
