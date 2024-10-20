use std::error::Error;

use super::{algorithm_even_fibonacci_numbers::EvenFibonacciNumbers, t_algorithm::Algorithm};

pub fn get_algorithm(name: String) -> Result<impl Algorithm, Box<dyn Error>>{
    match name.as_str() {
        "even_fibonacci_numbers" => Ok(EvenFibonacciNumbers::new()),
        _ => Err("invalid algorithm".into())
    }
}