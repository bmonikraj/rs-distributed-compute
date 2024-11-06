use std::error::Error;

use super::{concrete_algorithm_even_fib_sum::EvenFibSum, concrete_algorithm_primality_seq::PrimalitySeq, t_algorithm::Algorithm};

/*
even_fib_sum => https://projecteuler.net/problem=2
primality_seq => https://projecteuler.net/problem=216 
*/

pub fn get_algorithm(name: &String) -> Result<Box<dyn Algorithm + Send>, Box<dyn Error>> {
    match name.as_str() {
        "even_fib_sum" => Ok(Box::new(EvenFibSum::new())),
        "primality_seq" => Ok(Box::new(PrimalitySeq::new())),
        &_ => Err("unimplemented".into()),
    }
}
