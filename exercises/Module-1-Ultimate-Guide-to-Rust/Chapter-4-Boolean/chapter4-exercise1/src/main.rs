/*
 * Write a function that implements the logic, “Michael is allowed to drive if he’s 18 or older. Anyone else can drive if they are 16 or older.”
 */

fn can_drive(name: &str, age: u32) -> bool {
  (name == "Michael" && age >= 18) || (name != "Michael" && age >= 16)
}

fn main() {
    println!("{}", can_drive("Michael", 18));
}
