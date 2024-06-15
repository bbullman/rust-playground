/*
 Last time, we implemented a factorial function using recursion. As a reminder, a factorial multiplies all the numbers from 1 to the target, so fact(5) == 1 * 2 * 3 * 4 * 5. Write a fact function using iteration and a while loop instead of recursion.
 */

#[allow(unused_variables, unused_mut)]
fn fact(x: i32) -> i32 {
    let mut res: i32 = 1;
    let mut num = 1;
    while num <= x {
        res *= num;
        num += 1;
    }
    res
}

fn main() {
    println!("{}", fact(5));
}
