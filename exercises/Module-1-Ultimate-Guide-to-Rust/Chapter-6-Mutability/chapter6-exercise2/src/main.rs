/*
Write a program that uses nested while loops to produce this output:

X
XX
XXX
XXXX
XXXXX
XXXXXX
XXXXXXX
XXXXXXXX
XXXXXXXXX
XXXXXXXXXX
*/

#[allow(unused_variables, unused_mut)]
fn triangle(n: i32) {
    let mut outer: i32 = 1;
    while outer <= n {
        let mut inner: i32 = 1;
        while inner <= outer {
            print!("X");
            inner += 1;
        }
        println!("");
        outer += 1;
    }
}

fn main() {
  triangle(10)
}
