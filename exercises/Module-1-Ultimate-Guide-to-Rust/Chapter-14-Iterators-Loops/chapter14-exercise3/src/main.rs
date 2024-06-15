/*
 Use a for loop to write a function that prints the name given to it vertically. In addition, tell us the position of each letter. For example, if you give the function "Alice" it should give you the output:

A = 1 out of 5
l = 2 out of 5
i = 3 out of 5
c = 4 out of 5
e = 5 out of 5
*/

#[allow(unused_variables, unused_mut)]
fn print_vertical(name: &str) {
   let mut n = 1;
   for i in name.chars() {
      println!("{} = {} out of {}", i, n, name.len());
      n+=1;
   }
}

fn main() {
  print_vertical("Alice")
}
