use crate::model::response_health::ResponseHealth;

use super::t_health::THealth;

pub struct Health {

}

impl THealth for Health {
    fn new() -> Self {
        return Self{}
    }

    fn info(&self) -> ResponseHealth {
        let response = ResponseHealth{
            status: "up".to_string(),
        };
        return response;
    }
}