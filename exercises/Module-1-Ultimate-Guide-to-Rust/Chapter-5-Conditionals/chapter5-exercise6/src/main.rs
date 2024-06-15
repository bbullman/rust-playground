/*
Fix the program below so that it compiles and produces the output:

I NEED APPLES!!!
I NEED APPLES!!!
I have enough apples
*/

fn helper(apples: i32) {
    let x = if apples > 10 {
        "I have enough apples"
    } else {
        "I need APPLES!!!"
    };

    println!("{}", x);
}

fn main() {
    helper(9);
    helper(10);
    helper(11);
}
