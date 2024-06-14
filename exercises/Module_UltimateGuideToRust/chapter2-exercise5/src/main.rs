/*
  Look through the following program and answer these questions for yourself. Run the program to see if you were right.

    What do the expressions evaluate to?

    Will there be effects? If so, what?
*/

fn main() {
    println!("Far over the Misty Mountains cold");
    "Dragon!!!";
    let dwarves: i32 = 13;
    let hobbit: i32 = {
        "Not a wizard";
        5 - 4
    };
    dwarves - 3; // This is unused
    println!("The Company of Thorin has {} dwarves and {} hobbit, {} members in all.", dwarves, hobbit, dwarves + hobbit);
    println!("After the Battle of the Five Armies, there were only {} dwarves remaining.", dwarves - 3);
}
