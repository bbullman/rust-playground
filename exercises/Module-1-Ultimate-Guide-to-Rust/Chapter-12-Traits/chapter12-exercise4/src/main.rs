/*
 * Modify the stringify function (without modifying the main function) to use type parameters and trait bounds and make this program print Success!:
 */

fn stringify<T: std::fmt::Display>(x: T) -> String {
    format!("{}", x)
}

fn main() {
    assert_eq!(stringify(5), "5".to_owned());
    assert_eq!(stringify(true), "true".to_owned());
    assert_eq!(stringify("Hello"), "Hello".to_owned());
    println!("Success!");
}
