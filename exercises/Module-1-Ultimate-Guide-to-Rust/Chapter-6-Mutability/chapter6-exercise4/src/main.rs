/*
Using a while loop, write “99 bottles of beer on the wall.” The output should look like this:

99 bottles of beer on the wall,
99 bottles of beer.
You take one down, pass it around.
98 bottles of beer on the wall.

98 bottles of beer on the wall,
98 bottles of beer.
You take one down, pass it around.
97 bottles of beer on the wall.
...
2 bottles of beer on the wall,
2 bottles of beer.
You take one down, pass it around.
1 bottle of beer on the wall.

1 bottle of beer on the wall,
1 bottle of beer.
You take one down, pass it around.
No bottles of beer on the wall.
*/

fn print() {
    let mut iter: i32 = 99;
    while iter > 0 {
        if iter == 1 {
            print!("{} bottle of beer on the wall,\n{} bottle of beer.\nYou take it down, pass it around.\n", iter, iter);
        }
        else {
            print!("{} bottles of beer on the wall,\n{} bottles of beer.\nYou take one down, pass it around.\n", iter, iter);
        }
        iter -= 1;
        if iter == 0 {
            println!("No bottles of beer on the wall.");
        } else if iter == 1 {
            println!("{} bottle of beer on the wall.", iter);
            println!("");
        } else {
            println!("{} bottles of beer on the wall.", iter);
            println!("");
        }
    }
}


fn main() {
  print()
}
