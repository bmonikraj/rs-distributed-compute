use std::error::Error;

use crate::model::response_compute::ResponseCompute;

pub trait Algorithm {
    fn compute(&mut self, param: &String) -> Result<ResponseCompute, Box<dyn Error>>;
}
