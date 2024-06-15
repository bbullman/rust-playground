/*
Write a function that prints a border made out of asterisks (*) for a given number of rows and columns. For example, with the main function:

We should get the output:

*****
*   *
*   *
*   *
*   *
*****
*/

fn print_top_bottom(width: i32) {
    let mut iter: i32 = 1;
    while iter <= width {
        print!("*");
        iter += 1;
    }
    println!("");
}

fn print_middle(width: i32) {
    let mut outer: i32 = 1;
    while outer <= width - 1 {
        let mut inner: i32 = 1;
        while inner <= width {
            if inner == 1 || inner == width {
                print!("*")
            } else {
                print!(" ");
            }
            inner += 1;
        }
        println!("");
        outer += 1;
    }
}

fn print_border(num: i32) {
    print_top_bottom(num);
    print_middle(num);
    print_top_bottom(num);
}

fn main() {
    print_border(5);
}
