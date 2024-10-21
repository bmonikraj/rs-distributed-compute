use std::error::Error;

use crate::model::response_compute::ResponseCompute;

pub trait Algorithm {
    fn new() -> Self;
    fn compute(&mut self, request_compute: String) -> Result<ResponseCompute, Box<dyn Error>>;
}
