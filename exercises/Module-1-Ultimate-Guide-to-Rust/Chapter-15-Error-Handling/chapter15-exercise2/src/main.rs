/*
 The previous example produced output from the find_yellow_animal function. Instead, in this exercise, we’re going to return a value from the function. Implement the new find_yellow_animal so that the program compiles and passes the tests.
 */

#[derive(PartialEq, Eq, Debug)]
struct Animal {
    name: String,
    color: Color,
}

#[derive(PartialEq, Eq, Debug)]
enum Color {
    Red,
    Yellow,
    Blue,
    Brown,
    White,
}

#[allow(unused_variables, unused_mut)]
fn find_yellow_animal(animals: &[Animal]) -> Option<&Animal> {
    for animal in animals {
        if animal.color == Color::Yellow {
            return Some(animal);
        }
    }
    None
}

fn main() {
    let zoo = [
        Animal { name: "Fox".to_owned(), color: Color::Red },
        Animal { name: "Monkey".to_owned(), color: Color::Brown },
        Animal { name: "Polar Bear".to_owned(), color: Color::White },
        Animal { name: "Giraffe".to_owned(), color: Color::Yellow },
        Animal { name: "Canary".to_owned(), color: Color::Yellow },
        Animal { name: "Dolphin".to_owned(), color: Color::Blue },
    ];

    find_yellow_animal(&zoo);
}
