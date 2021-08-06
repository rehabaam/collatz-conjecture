use std::time::{SystemTime};

// Break the conjecture
fn main() {

    // Choose a number 
    let x = 3112412422142425353344343434424347435435435436346346443634627527527575230953665899508087094643634634431.0;

    // Starts the fun!!
    check_conjecture(x);
}

// validate Collatz conjecture 
fn check_conjecture(mut x:f64) {

    // Sstart the timer
    let t1 = SystemTime::now();
    println!("First number {}",x);

    loop {

        // Handle even cases
        if is_even(x) {

			// Print the input number
            println!("Is Even {}",x);

            // Divide by 2
			x = x / 2.0;

			// Print the new number
			println!("New half {}", x);

            continue;
        }

        // If it reaches 1, then stop, no need to get stuck in a forever loop
		if x == 1.0 {

			// Print the time taken for reaching 1
			let t2 = t1.elapsed();
			println!("Time taken to reach '1': {:?}", t2);
			break;
		}

        // Print the input number
		println!("Is Odd {}", x);

		// Do 3x+1 magic
		x = 3.0*x + 1.0;

		// Print the new number
		println!("new Even {}", x);
    }

}

// Just returns even or odd
fn is_even(x:f64) -> bool {
    x % 2.0 == 0.0
}