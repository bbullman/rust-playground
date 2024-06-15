/*
 * Rewrite the main function to not use any variables. You’ll still want to call both the double and triple function.
 */

fn double(x: i32) -> i32 {
    x * 2
}

fn triple(x: i32) -> i32 {
    x * 3
}

fn main() {
    println!("Answer: {}", triple(double(5)));
}
