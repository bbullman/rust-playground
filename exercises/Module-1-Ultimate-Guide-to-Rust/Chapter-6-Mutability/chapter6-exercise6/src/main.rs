/*
 * Write a program that prints the numbers 10 to 1.
 */

fn print() {
   let mut iter: i32 = 10;
   while iter >= 1 {
      println!("{}", iter);
      iter -= 1;
   }
}

fn main() {
  print()
}
