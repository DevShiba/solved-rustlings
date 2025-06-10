#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // TODO: Fix the compiler error by adding something to this match statement.
    match optional_point { //Adding & before optional_point allows us to match against the Option type too.
        Some(ref p) => println!("Co-ordinates are {},{}", p.x, p.y), // Adding ref allows us to borrow the Point inside the Option without taking ownership. That solves the exercise.
        _ => panic!("No match!"),
    }

    println!("{optional_point:?}"); // Don't change this line.
}
