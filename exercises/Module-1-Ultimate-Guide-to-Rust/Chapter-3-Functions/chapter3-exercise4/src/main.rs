/*
 * Add the helper functions needed to make this program pass. Do not modify the main function
 * itself.
 */

fn main() {
    let x = (5 + 3) * (6 + 4);
    let y = times(add_3(5), add_4(6));
    assert_eq!(x, y);
    println!("Good job!");
}

fn add_3(a: i32) -> i32 {
    a + 3
}

fn add_4(a: i32) -> i32 {
    a + 4
}

fn times(a: i32, b: i32) -> i32 {
    a * b
}
