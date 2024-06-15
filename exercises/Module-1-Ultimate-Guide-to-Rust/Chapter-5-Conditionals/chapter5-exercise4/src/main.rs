/*
 Flesh out the function, is_hot, which always returns the opposite of is_cold. 
*/

fn is_hot() -> bool {
    !is_cold() // your code goes here!
}
fn is_cold() -> bool {
    true // feel free to change this to false
}

fn main() {
  println!("{}", is_hot())
}
