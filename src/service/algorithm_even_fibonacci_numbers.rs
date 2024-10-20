use crate::model::{response_compute::ResponseCompute, request_compute::RequestCompute};
use std::{error::Error};
use super::t_algorithm::Algorithm;

pub struct EvenFibonacciNumbers {
    n: i128,
}

impl Algorithm for EvenFibonacciNumbers {
    fn new() -> Self {
        return Self{n: 0};
    }

    fn compute(&mut self, param: String) -> Result<ResponseCompute, Box<dyn Error>> {
        
        self.n = param.parse::<i128>().unwrap();
        let mut r = ResponseCompute{name: "EvenFibonacciNumbers".to_string(), result: "".to_string()};
        
        let m: i128 = -1;

        if m < 0 {
            Err("Error".into())
        } else {
            r.result = m.to_string();
            Ok(r)
        }
    }
}