/*
 * Rewrite the program below so it doesn’t use any lets. Reminder: you can put any expression inside a macro call’s parameter list.
 */

fn comment(apples: i32) {


    println!("You have {} apples",
        if apples > 10 {
            "lots of"
        } else {
            "very few"
        }
    );
}

fn main() {
    comment(5);
    comment(100);
}
