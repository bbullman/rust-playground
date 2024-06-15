/*
 The .rev() method call allows us to iterate backward through an Iterator. Using this, write a function that prints a name vertically in reverse order. For example, if you give the function "Alice", you should get this output:
 */

#[allow(unused_variables, unused_mut)]
fn vertical_backwards_name(name: &str) {
   for i in name.chars().rev() {
      println!("{}", i);
   }
}

fn main() {
  vertical_backwards_name("Alice")
}
