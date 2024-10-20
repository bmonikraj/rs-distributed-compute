use std::error::Error;

use crate::model::{request_compute::RequestCompute, response_compute::ResponseCompute};

pub trait Algorithm {
    fn new() -> Self;
    fn compute(&mut self, request_compute: String) -> Result<ResponseCompute, Box<dyn Error>>;
}