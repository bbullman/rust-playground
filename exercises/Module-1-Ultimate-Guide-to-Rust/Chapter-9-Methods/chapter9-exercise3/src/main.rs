/*
 The pow method lets you raise a number to a certain power. For example, 2_i32.pow(2) will compute two squared. Can you test what five squared plus three cubed equals by implementing the compute_power function? (The a and b getting passed in are the powers we want to apply.)
 */

#[allow(unused_variables, unused_mut)]
fn compute_power(a: u32, b: u32) -> i32{
  5_i32.pow(a) + 3_i32.pow(b)
}

fn main() {
  println!("{}", compute_power(2, 4))
}

