/*
 Modify the add_one method to take a &mut self and modify the value in place. You’ll need to change the trait method’s parameter list and return type, the two trait implementations, and the call site inside the older method.
 */

struct Person<Name, Age> {
    name: Name,
    age: Age,
}

impl<Name, Age> Person<Name, Age>
where
    Name: std::fmt::Display,
    Age: std::fmt::Display,
{
    fn greet(&self) {
        println!("Hello, {}. You are {} years old.", self.name, self.age);
    }
}

trait AddOne {
    fn add_one(&mut self);
}

impl AddOne for u32 {
    fn add_one(&mut self){
        *self += 1
    }
}

impl AddOne for u64 {
    fn add_one(&mut self) {
        *self += 1
    }
}

impl<Name, Age: AddOne> Person<Name, Age> {
    fn older(&mut self) {
        self.age.add_one();
    }
}

fn main() {
    let mut alice: Person<String, u32> = Person {
        name: "Alice".to_owned(),
        age: 30_u32,
    };
    alice.older();
    alice.greet();
    
    let mut bob: Person<String, u64> = Person {
        name: "Bob".to_owned(),
        age: 35_u64,
    };
    bob.older();
    bob.greet();
}
