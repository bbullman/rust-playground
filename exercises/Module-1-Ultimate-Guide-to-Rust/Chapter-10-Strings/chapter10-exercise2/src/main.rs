/*
 Numbers allow you to use + to add them together, or += to mutate a variable. The same applies to Strings. Fix this program. Again, try to figure out what the problem is without asking the compiler.
 */

fn main() {
    let hello = "Hello, ".to_owned() + "world!";
    println!("{}", hello);
}
