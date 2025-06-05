fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???
    let a = [1; 100];  // This creates an array of 100 elements, all initialized to 1. That solves the exercise.

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
