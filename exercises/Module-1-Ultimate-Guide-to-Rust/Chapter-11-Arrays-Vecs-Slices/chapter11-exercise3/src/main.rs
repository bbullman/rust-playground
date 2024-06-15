/*
 The Fibonacci sequence starts with the numbers 1 and 1. Then, each number after that is the sum of the previous two numbers. For example, the next number is 2 (1 + 1), followed by 3 (1 + 2), followed by 5 (2 + 3).

Fill out the fibs function so that it produces a vector of Fibonacci numbers of the given size.
*/

#[allow(unused_variables, unused_mut)]
fn fibs(count: usize) -> Vec<u32> {
   if count == 1 {
      return vec![1];
   }
   let mut vec = vec![1, 1];
   let mut first_index = 0;
   let mut second_index = 1;
   while vec.len() < count {
      vec.push(vec[first_index] + vec[second_index]);
      first_index += 1;
      second_index += 1;
   }
   vec
}

fn main() {
  let res: Vec<u32> = fibs(10);
  println!("{:?}", res);
}
