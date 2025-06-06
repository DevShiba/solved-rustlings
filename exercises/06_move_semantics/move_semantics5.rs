#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: &String) -> char { // I added `&` to borrow the string
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) { // I removed the `&` to take ownership of the string
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data); // Used & to borrow the string

    string_uppercase(data);
}
