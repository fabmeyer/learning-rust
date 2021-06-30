use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::time::{Instant};
use std::mem::replace;

fn main() {
    let start = Instant::now();
    for _i in 0..1000 {
    
        let mut fib1: BigUint = Zero::zero();
        let mut fib2: BigUint = One::one();
    
        let mut nth_fib = 1;
    
        while nth_fib < 1000 {
            // println!("{}. fibonacci number is: {}", nth_fib, fib1);
            let temp: BigUint = &fib1 + &fib2;
            // This is a low cost way of swapping fib1 with fib2 and fib2 with temp.
            fib1 = replace(&mut fib2, temp);
            nth_fib += 1;
        }
    }


    let duration = start.elapsed();
    println!("Time elapsed: {} ms", duration.as_millis());
}
