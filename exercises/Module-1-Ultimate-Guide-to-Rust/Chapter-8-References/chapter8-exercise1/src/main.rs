/*
 * The is_five function below has a bug. You can fix it by either adding in a borrow (&) or deref (*). Try fixing it both ways.
 */

fn is_five(x: &i32) -> bool {
    x == &5
}

fn main() {
    assert!(is_five(&5));
    assert!(!is_five(&6));
    println!("Success!");
}
