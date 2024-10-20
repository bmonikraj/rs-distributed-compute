use crate::model::response_health::ResponseHealth;

pub struct Health;

impl Health {
    pub fn new() -> Self {
        return Self{}
    }

    pub fn info(&self) -> ResponseHealth {
        let response = ResponseHealth{
            status: "up".to_string(),
        };
        return response;
    }
}