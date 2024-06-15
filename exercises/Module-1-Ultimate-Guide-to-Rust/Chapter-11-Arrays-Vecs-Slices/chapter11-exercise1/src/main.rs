/*
 * Fill out an implementation of the maximum function to return the maximum value in the provided slice. If provided with an empty slice, you can return 0
 */

#[allow(unused_variables, unused_mut)]
fn maximum(numbers: &[u32]) -> u32 {
   let mut max: u32 = 0;
   let mut i = 0;
   while i < numbers.len() {
      if numbers[i] > max {
         max = numbers[i];
      }
      i += 1;
   }
   max
}

fn main() {
    println!("{}", maximum(&[1,2,3,4]));
}
