/*
 I want to write a minus function that subtracts two unsigned numbers. If the result would be negative, I want to return a None instead. Modify the result type of minus and its implementation so this program succeeds, without changing main.}
 */

fn minus(x: u32, y: u32) -> Option<u32> {
    if x >= y {
        Some(x - y)
    } else {
        None
    }
}

fn main() {
    assert_eq!(minus(5, 2), Some(3));
    assert_eq!(minus(3, 2), Some(1));
    assert_eq!(minus(2, 2), Some(0));
    assert_eq!(minus(1, 2), None);

    println!("Success!");
}
