/*
 The following program won’t compile due to a borrow error. Try to figure out what the error is before asking the compiler. Then fix the error by rearranging two statements.
 */

fn main() {
    let mut x: i32 = 5;
    let y: &mut i32 = &mut x;

    *y += 1;

    assert_eq!(y, &6);
    assert_eq!(x, 6);
    println!("Success!");
}
