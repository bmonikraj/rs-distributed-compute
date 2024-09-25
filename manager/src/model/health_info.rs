use serde::Serialize;

#[derive(Serialize)]
pub struct InfoResponse {
    status: String,
}

pub trait TraitInfoResponse {
    fn new() -> Self;
    fn set(&mut self, status: String);
}

impl TraitInfoResponse for InfoResponse {
    fn new() -> Self {
        let obj = InfoResponse {
            status: "up".to_string(),
        };
        return obj;
    }
    
    fn set(&mut self, status: String) {
        self.status = status;
    }
}