/*
 * Fix the call to the add function inside main below.a
 */

fn add(x: &i32, y: &i32) -> i32 {
    *x + *y
}

fn main() {
    let result = add(&5, &6);
    assert_eq!(result, 11);
    println!("Success!");
}
