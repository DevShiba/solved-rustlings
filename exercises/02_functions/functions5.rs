// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i32 {
    num * num // Removing the ; at the end of this line to fix the function body. That solves the exercise.
    // Note: The semicolon at the end of the line was causing the function to return `()`, which is the unit type in Rust.
    // return num * num; // Alternatively, you could use a return statement.
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}
