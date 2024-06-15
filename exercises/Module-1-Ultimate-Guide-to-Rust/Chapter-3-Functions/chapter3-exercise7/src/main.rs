/*
  Define a quadruple function, which results in the input value multiplied by 4, built by calling the double function twice. To help you out, here’s a skeleton that should work.
 */

#[allow(dead_code)] // allows to turn off warnings for unused imports
fn double(x: i32) -> i32 {
    x * 2
}

#[allow(unused_variables, unused_mut)]
fn quadruple(x: i32)-> i32 {
    double(double(x))
}

fn main() {
  println!("{}", quadruple(4))
}
