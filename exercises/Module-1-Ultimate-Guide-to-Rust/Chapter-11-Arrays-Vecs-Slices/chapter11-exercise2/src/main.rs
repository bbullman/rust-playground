/*
 Fill out an implementation of the reverse function that takes a slice of i32s and returns a Vec with the numbers in reverse order. Note that you’ll want to use the push method and a while loop that moves backwards through the numbers parameter:
 */

#[allow(unused_variables, unused_mut)]
fn reverse(numbers: &[i32]) -> Vec<i32>{
   let mut vec: Vec<i32> = vec![];
   let mut cnt = numbers.len();
   while cnt > 0 {
      cnt -= 1;
      vec.push(numbers[cnt]);
   }
   vec
}

fn main() {
  let res: Vec<i32> = reverse(&[1,2,3,4,5]);
  println!("{:?}", res)
}
