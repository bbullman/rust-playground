/*
 * Modify the program above so that greet works for more than just a String for the name. You can test it with this main function:
 */

fn greet<Name: std::fmt::Display, Age: std::fmt::Display>(person: &Person<Name, Age>) {
    println!("Hello, {} you are {} years old.", person.name, person.age);
}

fn main() {
    let alice = Person {
        name: "Alice".to_owned(),
        age: 30_u32,
    };
    greet(&alice);

    let bob = Person {
        name: "Bob",
        age: 35_u64,
    };
    greet(&bob);
}

struct Person<Name, Age> {
    name: Name,
    age: Age,
}

