use std::error::Error;

use super::{concrete_algorithm_even_fib_n::EvenFibonacciNumbers, t_algorithm::Algorithm};

pub fn get_algorithm(name: String) -> Result<Box<impl Algorithm>, Box<dyn Error>> {
    match name.as_ref() {
        "even_fib_n" => Ok(Box::new(EvenFibonacciNumbers::new())),
        &_ => Err("unimplemented".into())
    }
}