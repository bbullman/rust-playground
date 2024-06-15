/*
 * Make the program below compile.
 */

fn say_hi(name: &str) {
    println!("Hi {}", name);
}

fn main() {
    let name = "Michael".to_owned();
    say_hi(&name);
    say_hi(&name);
}
