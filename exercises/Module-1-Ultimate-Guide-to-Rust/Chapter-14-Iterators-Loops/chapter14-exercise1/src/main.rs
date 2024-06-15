/*
 * Implement a factorial using a for loop.
 */

#[allow(unused_variables, unused_mut)]
fn fact(x: u32) -> u32 {
    let mut res = 1;
    for i in 1..=x {
        res *= i;
    }
    res
}

fn main() {
  println!("{}", fact(10))
}
