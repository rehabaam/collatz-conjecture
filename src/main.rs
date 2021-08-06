extern crate num;
use std::time::{SystemTime};
use num::bigint::{BigInt, Sign};

// Break the conjecture
fn main() {

    // Choose a number 
    let x = BigInt::new(Sign::Plus, vec![10, 1]);
    let f = num::pow(x, 300);
    
    // Starts the fun!!
    check_conjecture(f);
}

// validate Collatz conjecture 
fn check_conjecture(mut x:BigInt) {

    let one = BigInt::from(1);
    let zero = BigInt::from(0);

    // Sstart the timer
    let t1 = SystemTime::now();
    println!("First number {}",x);

    loop {

        // Handle even cases
        if is_even(x.clone(), zero.clone()) {

			// Print the input number
            println!("Is Even {}",x);

            // Divide by 2
            x = x.clone() / 2;

			// Print the new number
			println!("New half {}", x);
            continue;
        }

        // If it reaches 1, then stop, no need to get stuck in a forever loop
		if x == one {

			// Print the time taken for reaching 1
			let t2 = t1.elapsed();
			println!("Time taken to reach 1: {:?}", t2);
			break;
		}

        // Print the input number
		println!("Is Odd {}", x);

		// Do 3x+1 magic
		x =(x.clone() * 3) + 1;

		// Print the new number
		println!("new Even {}", x);
    }

}

// Just returns even or odd
fn is_even(x:BigInt, zero:BigInt) -> bool {
    x % 2 == zero
}