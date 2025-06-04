// TODO: Add the missing type of the argument `num` after the colon `:`.
fn call_me(num: u32) { // I added `u32` to specify the type of `num` as an unsigned 32-bit integer. That solves the exercise.
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
