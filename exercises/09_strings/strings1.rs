// TODO: Fix the compiler error without changing the function signature.
fn current_favorite_color() -> String {
    "blue".to_string() // Converts the string literal "blue" into a String type.
    // String::from("blue")
    // format!("blue")

    // That resolves the exercise.
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
