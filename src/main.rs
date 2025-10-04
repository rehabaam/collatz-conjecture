///////////////////////////////////////////////////////////////////////////////////////
//                                                                                   //
//                          Collatz conjecture (known as 3x+1)                       //
//                                                                                   //
///////////////////////////////////////////////////////////////////////////////////////

extern crate num;
use std::time::SystemTime;
use num::bigint::{BigInt, Sign};

// Break the conjecture
fn main() {
    // Choose a number 
    let x = BigInt::new(Sign::Plus, vec![10, 1]);
    let f = num::pow(x, 300);
    
    // Starts the fun!!
    check_conjecture(f);
}

// Validate Collatz conjecture 
fn check_conjecture(mut x: BigInt) {
    let one = BigInt::from(1);
    let two = BigInt::from(2);
    let three = BigInt::from(3);

    // Start the timer
    let t1 = SystemTime::now();
    let mut steps = 0u64;
    
    println!("Starting number: {}", x);
    println!("----------------------------------------");

    loop {
        // If it reaches 1, then stop
        if x == one {
            let t2 = t1.elapsed();
            println!("----------------------------------------");
            println!("Reached 1 in {} steps", steps);
            println!("Time taken: {:?}", t2);
            break;
        }

        steps += 1;

        // Handle even cases - use bit shift for efficiency
        if is_even(&x) {
            x /= &two;
        } else {
            // Handle odd cases - 3x + 1
            let next = &x * &three + &one;
            x = next;
        }
    }
}

// Check if number is even using modulo operation
fn is_even(x: &BigInt) -> bool {
    (x % 2) == BigInt::from(0)
}
