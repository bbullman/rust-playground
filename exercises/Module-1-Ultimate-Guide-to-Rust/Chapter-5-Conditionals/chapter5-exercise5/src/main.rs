/*
 The exercise will be a reminder of both Booleans and cover conditionals. I want to write a program that tells the kids whether or not they can go outside. The kids are allowed to go outside if it’s not raining, or if the temperature is at least 10 degrees. Complete the program below by replacing the unimplemented!() macro calls with the correct logic. Do not modify the main function. The output should be:

 Sorry, it's too cold and it's raining
 You can go outside
 You can go outside
 You can go outside
*/ 

fn can_go_outside(is_raining: bool, temp: i32) -> bool {
    if !is_raining || temp >= 10 {
        true
    } else {
        false
    }
}

fn tell_kids(is_raining: bool, temp: i32) {
    let msg: &str = if can_go_outside(is_raining, temp) {
        "You can go outside"
    } else {
        "Sorry it's too cold and it's raining"
    };

    println!("{}", msg);
}

fn main() {
    tell_kids(true, 9);
    tell_kids(true, 10);
    tell_kids(false, 9);
    tell_kids(false, 10);
}
