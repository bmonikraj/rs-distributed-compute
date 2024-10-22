use std::error::Error;

use super::{concrete_algorithm_even_fib_n::EvenFibN, t_algorithm::Algorithm};

/*
even_fib_n => https://projecteuler.net/problem=2
*/

pub fn get_algorithm(name: String) -> Result<Box<impl Algorithm>, Box<dyn Error>> {
    match name.as_ref() {
        "even_fib_n" => Ok(Box::new(EvenFibN::new())),
        &_ => Err("unimplemented".into()),
    }
}
