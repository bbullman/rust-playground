/*
 * Print multiples of 5 from 5 to 100.
 */

fn print() {
   let mut iter: i32 = 5;
   while iter <= 100 {
      println!("{}", iter);
      iter += 5;
   }
}

fn main() {
  print()
}
