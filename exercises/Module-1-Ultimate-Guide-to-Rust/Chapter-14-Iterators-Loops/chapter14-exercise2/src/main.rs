/*
 * Using for loops, write a function that prints out borders. With the main function:
 */

fn print_border(x: u32, y: u32) {
    print_top_bottom(x);
    for i in 1..y {
        print_middle(x);
    }
    print_top_bottom(x);


}

fn print_top_bottom(x: u32) {
    for i in 1..x {
        print!("*");
    }
    println!("");
}

fn print_middle(x: u32) {
    for i in 1..x {
        if i == 1 || i == x - 1 {
            print!("*");
        } else {
            print!(" ")
        }
    }
    println!("");
}

fn main() {
    print_border(6, 5);
}
