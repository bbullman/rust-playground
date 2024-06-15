/*
 Implement the total_length function by using the len method on a string literal to compute the combined length of the two strings "Hello" and "World". Try it out with "Hi" and "Educative" too.
 */

#[allow(unused_variables, unused_mut)]
fn total_length(a: &str, b: &str) -> usize {
  a.len() + b.len()
}

fn main() {
  println!("{}", total_length("Hello", "World"))
}
