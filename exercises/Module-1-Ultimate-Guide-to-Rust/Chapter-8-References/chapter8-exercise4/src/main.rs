/*
 * Fix the program below. Ideally, figure out what the problem is before asking the compiler what the problem is.
 */

fn main() {
    let mut x: i32 = 5;
    let y: &mut i32 = &mut x;

    *y += 1;

    assert_eq!(x, 6);
    println!("Success!");
}
