fn bigger(a: i32, b: i32) -> i32 {
    // TODO: Complete this function to return the bigger number!
    // If both numbers are equal, any of them can be returned.
    // Do not use:
    // - another function call
    // - additional variables
    if a > b {
        return a
    }
    b
    // I am using an `if` statement to compare the two numbers.
    // If `a` is greater than `b`, I return `a`.
    // If `b` is greater than or equal to `a`, I return `b`.
    // I used early return to exit the function immediately when `a` is greater than `b`.
    // This solve the exercise.
}

fn main() {
    bigger(10, 8); // This line is just to call the function and test it.
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
