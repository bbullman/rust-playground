
/*
 I’m looking for a yellow animal at the zoo. When I find it, I want to stop looking. Implement the find_yellow_animal function so that main compiles and produces the following output:

The Fox is not yellow.
The Monkey is not yellow.
The Polar Bear is not yellow.
I found a yellow Giraffe. Now I'm done looking.

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

fn find_yellow_animal(animals: &[Animal]) {
    for animal in animals {
        if animal.color == Color::Yellow {
            println!("I found a yellow {}. Now I'm done looking.", animal.name);
            break;
        } else {
            println!("The {} is not yellow.", animal.name);
        }
    }
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
