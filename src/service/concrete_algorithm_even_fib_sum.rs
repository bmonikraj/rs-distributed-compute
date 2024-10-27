use super::t_algorithm::Algorithm;
use crate::model::response_compute::ResponseCompute;
use std::error::Error;

pub struct EvenFibSum {
    n: i128,
}

impl Algorithm for EvenFibSum {
    fn compute(&mut self, param: String) -> Result<ResponseCompute, Box<dyn Error>> {
        self.n = param.parse::<i128>().unwrap();
        let mut r = ResponseCompute {
            name: "EvenFibSum".to_string(),
            result: "".to_string(),
            duration_micro_secs: 0,
        };

        let sum_after_compute = self.sum_even_terms_of_fibonacci_upto_n(self.n);

        if sum_after_compute < 0 {
            Err(format!("error while computing sum of even valued terms of fibonacci series upto n = {}", self.n).into())
        } else {
            r.result = sum_after_compute.to_string();
            Ok(r)
        }
    }
}

impl EvenFibSum {
    pub fn new() -> Self {
        return Self { n: 0 };
    }

    fn sum_even_terms_of_fibonacci_upto_n(&self, n: i128) -> i128 {
        let mut result = std::i128::MIN;

        if n < 1 {
            log::error!("input is less than 1, which is invalid for fibonacci sequence");
            return result;
        }

        if n == 1 {
            result = 0;
            return result;
        }

        if n == 2 {
            result = 2;
            return result;
        }

        let mut x: i128 = 1;
        let mut y: i128 = 2;
        let mut z: i128;
        result = y;

        for _ in 0..n-2 {
            if x > 4000000 || y > 4000000 {
                log::warn!("x={} y={} overflow will happen for next fibonnaci number", x, y);
                break;
            }
            z = x + y;
            if self.is_even(z) {
                result += z;
            }
            x = y;
            y = z;
        }

        return result;
    }

    fn is_even(&self, n: i128) -> bool {
        if n % 2 == 0 {
            return true;
        } 
        return false;
    }
}
