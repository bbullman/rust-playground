/*
 * Replace all four FIXMEs below with the correct type:
 */

fn want_apples() -> bool {
    true
}

fn want_five() -> bool {
    false
}

fn talk_about_fruit() -> bool {
    true
}

fn talk_about_numbers() -> bool {
    false
}

fn main() {
    let fruit: &str = if want_apples() {
        "apple"
    } else {
        "banana"
    };

    let number: i32 = if want_five() {
        5
    } else {
        6
    };

    let _x: () = if talk_about_fruit() {
        println!("The fruit is {}", fruit);
    };
    
    let _y: () = if talk_about_numbers() {
        println!("The number is {}", number);
    };
}
