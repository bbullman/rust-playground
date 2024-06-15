/*
 I want to know more about Alice, but I may not know her age and height. display method so that the program compiles and runs correctly. Hint: what symbol does this question end with?
 */

struct Person {
    name: String,
    age: Option<u32>,
    height: Option<u32>,
}

impl Person {
    fn display(&self) -> Option<String> {
        Some(format!(
            "{} is {} years old and {}cm tall",
            self.name, self.age?, self.height?
        ))
    }
}

fn main() {
    let mut alice = Person {
        name: "Alice".to_owned(),
        age: None,
        height: None,
    };

    assert_eq!(alice.display(), None);

    alice.age = Some(30);
    assert_eq!(alice.display(), None);

    alice.height = Some(170);
    assert_eq!(
        alice.display(),
        Some("Alice is 30 years old and 170cm tall".to_owned())
    );

    alice.age = None;
    assert_eq!(alice.display(), None);

    println!("Success");
}
