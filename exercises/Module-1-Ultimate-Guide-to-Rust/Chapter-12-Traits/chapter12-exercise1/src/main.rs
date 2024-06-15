/*
 We previously had the program below, and said that we would be polite and not mention someone’s age. Modify the greet function so that it says how old the person is.
 */

struct Person<Name, Age> {
    name: Name,
    age: Age,
}

fn greet<Age: std::fmt::Display>(person: &Person<String, Age>) {
    println!("Hello, {} you are {} years old.", person.name, person.age);
}

fn main() {
    let alice: Person<String, u32> = Person {
        name: "Alice".to_owned(),
        age: 30_u32,
    };
    greet(&alice);

    let bob = Person {
        name: "Bob".to_owned(),
        age: 35_u64,
    };
    greet(&bob);
}
