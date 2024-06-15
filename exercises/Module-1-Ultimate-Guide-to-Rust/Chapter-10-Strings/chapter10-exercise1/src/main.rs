/*
 * Fix the program below so that it compiles and produces the output, “Hello, world!”. Try to fix it without asking the compiler for a hint.
 */

fn main() {
    let hello = "Hello, ";
    let world = "world!";
    let helloworld = format!("{}{}", hello, world);
    println!("{}", helloworld);
}
