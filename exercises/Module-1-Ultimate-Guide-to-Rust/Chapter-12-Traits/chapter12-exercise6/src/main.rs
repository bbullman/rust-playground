/*
 A previous exercise had you write a reverse function that was specialized to slices of i32s. Now that we know about type parameters and traits, generalize the reverse function below so that the program will succeed.
 */

fn reverse<T: Copy>(items: &[T]) -> Vec<T> {
    let mut res = vec![];
    let mut i = items.len();
    while i > 0 {
        i -= 1;
        res.push(items[i]);
    }
    res
}

fn main() {
    assert_eq!(reverse(&[4, 5, 2, 8]), &[8, 2, 5, 4]);
    assert_eq!(reverse(&[true, false, true, false]), &[false, true, false, true]);
    assert_eq!(reverse(&[(), ()]), &[(), ()]);
    println!("Success!");
}
