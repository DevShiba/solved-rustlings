fn main() {
    let x = 5; // This line declares a variable `x` and assigns it the value 5 and the type is inferred to be `i32`.
    let y: i8 = 10; // This line declares a variable `y` with an explicit type annotation of `i8` and assigns it the value 10.
    let z: u8 = 20; // This line declares a variable `z` with an explicit type annotation of `u8` and assigns it the value 20.
 
    // That solves the exercise
    println!("x has the value {x}");

    // Bonus: Print the values of `y` and `z`
    println!("y can represent values from -128 to 127, and it has the value {y}");
    println!("z can represent values from 0 to 255, and it has the value {z}");
}
