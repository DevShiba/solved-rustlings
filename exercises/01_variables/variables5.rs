fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    let number = 3; // This line re-declares `number` as an integer, shadowing the previous string variable.
    println!("Number plus two is: {}", number + 2);
}
