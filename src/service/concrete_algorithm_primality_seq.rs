use std::usize;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::model::response_compute::ResponseCompute;

use super::t_algorithm::Algorithm;


pub struct PrimalitySeq {
    n: i128,
}

impl Algorithm for PrimalitySeq {
    fn compute(&mut self, param: String) -> Result<crate::model::response_compute::ResponseCompute, Box<dyn std::error::Error>> {
        self.n = param.parse::<i128>().unwrap();
        let mut r = ResponseCompute {
            name: "PrimalitySeq".to_string(),
            result: "".to_string(),
            duration_micro_secs: 0,
        };

        let num_of_primes = self.count_primes_upto_seq(self.n);

        if num_of_primes == 0 {
            Err(format!("error while computing numbers of primes upto sequence 2n^2-1 for n = {}", self.n).into())
        } else {
            r.result = num_of_primes.to_string();
            Ok(r)
        }
    }
}

impl PrimalitySeq {
    pub fn new() -> Self {
        return Self { n: 0 };
    }

    fn count_primes_upto_seq(&self, n: i128) -> usize {
        let mut counter: usize = usize::MIN;
        if n <= 1 {
            log::error!("input is less than 2, this is not valid for seq 2*(n^2)-1");
            return counter;
        }

        let mut elements = vec![];
        for k in 2..n+1 {
            elements.push(k);
        }

        let filtered_numbers: Vec<_> = elements.par_iter().filter(|&x| self.check_prime(self.seq_num(*x))).collect();
        counter = filtered_numbers.len();
        return counter;
    }

    fn seq_num(&self, n: i128) -> i128 {
        return 2 * (n * n) - 1;
    }

    fn check_prime(&self, n: i128) -> bool {
        if n > 1 {
            for k in 2..n {
                if (n % k) == 0 {
                    return false;
                }
            }
            return true;
        } else {
            return false;
        }
    }
}