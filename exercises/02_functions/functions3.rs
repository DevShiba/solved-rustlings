fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // TODO: Fix the function call.
    call_me(5); // I changed the argument to 5, which is a valid u8 value. That solves the exercise.
    // Bonus: Call the function with a different value.
    call_me(3); // Calling the function with a different value to demonstrate its functionality.
}
