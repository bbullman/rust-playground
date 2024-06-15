/*
 * What is the output of this program?
 */

fn main() {
    let x = 5;
    {
        let x = 6;
        println!("First time: {}", x);
    }
    println!("Second time: {}", x);
}
