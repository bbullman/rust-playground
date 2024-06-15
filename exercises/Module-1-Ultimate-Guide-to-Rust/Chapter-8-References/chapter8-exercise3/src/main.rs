/*
 The program below compiles (with warnings), but panics because the assertion fails. Modify both the triple function and the call to triple in main so that the warnings go away and the program prints, “Success.”
*/

fn triple(x: &mut i32) {
    *x *= 3;
}

fn main() {
    let mut x = 5;
    triple(&mut x);
    assert_eq!(x, 15);
    println!("Success!");
}
