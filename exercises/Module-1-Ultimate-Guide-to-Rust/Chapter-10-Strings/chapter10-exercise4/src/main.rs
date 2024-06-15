/*
 * Write the function add_snoyman and say_hi to make this program compile:
 */

fn add_snoyman(name: &str) -> String {
    name.to_owned() + " Snoyman"
}

fn say_hi(name: &str) {
    println!("Hi {}", name);
}

fn main() {
    let michael: String = add_snoyman("Michael");
    let miriam: String = add_snoyman("Miriam");
    let john: &'static str = "John Doe";

    say_hi(&michael);
    say_hi(&miriam);
    say_hi(john);
}
