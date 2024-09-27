use crate::model::response_health::ResponseHealth;

pub trait THealth {
    fn new() -> Self;
    fn info(&self) -> ResponseHealth;
}