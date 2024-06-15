/*
 * Write a Double trait and implementations for enough types so that the following works:
 */

trait Double {
    type SignedInt;
    fn double(&self) -> Self::SignedInt;
}

impl Double for i32 {
    type SignedInt = i32;
    fn double(&self) -> Self::SignedInt {
        *self * 2
    }
}

impl Double for i64 {
    type SignedInt = i64;
    fn double(&self) -> Self::SignedInt {
        *self * 2
    }
}


fn main() {
    assert_eq!(10, 5_i32.double());
    assert_eq!(10, 5_i64.double());
    println!("Success!");
}
