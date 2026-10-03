///////////////////////////////////////////////////////////////////////////////////////
//                                                                                   //
//                          Collatz conjecture (known as 3x+1)                       //
//                                                                                   //
///////////////////////////////////////////////////////////////////////////////////////

use std::time::Instant;
use num::bigint::{BigInt, Sign};
use num::pow;
use num::Integer;

fn main() {
    // (2^32 + 10)^300, same starting number as before so runs stay comparable
    let base = BigInt::new(Sign::Plus, vec![10, 1]);
    let start = pow(base, 300);
    check_conjecture(start);
}

fn check_conjecture(mut x: BigInt) {
    let one = BigInt::from(1u32);
    let two = BigInt::from(2u32);
    let three = BigInt::from(3u32);

    let t1 = Instant::now();
    let mut steps = 0u64;

    println!("Starting number: {}", x);
    println!("----------------------------------------");

    loop {
        if x == one {
            let elapsed = t1.elapsed();
            println!("----------------------------------------");
            println!("Reached 1 in {} steps", steps);
            println!("Time taken: {:?}", elapsed);
            break;
        }

        steps += 1;

        if x.is_even() {
            x /= &two;
        } else {
            x = &x * &three + &one;
        }
    }
}
