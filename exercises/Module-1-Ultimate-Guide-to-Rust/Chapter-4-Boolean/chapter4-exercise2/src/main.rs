/*
 * Write a function that implements the logic, “You can see the movie if you are 17 or older, or you’re 13 or older and have a parent’s permission.”
 */


fn can_see_movie(age: i32, permission: bool) -> bool {
    age >= 17 || (age >= 13 && permission == true)
}

fn main() {
    println!("{}", can_see_movie(18, true));
}
