/*
 * Modify this program to get rid of all let statements. Remember: struct expressions are just
 * another kind of expression!A
 */

struct Fruit {
    apples: i32,
    bananas: i32,
}

fn increase_fruit(mut fruit: Fruit) -> Fruit {
    fruit.apples *= 2;
    fruit.bananas *= 3;
    fruit
}

fn new_fruit() -> Fruit {
    Fruit {
        apples: 10,
        bananas: 5,
    }
}

fn print_fruit(fruit: Fruit) {
    println!("You have {} apples and {} bananas", fruit.apples, fruit.bananas);
}

fn main() {
    print_fruit(increase_fruit(new_fruit()));
}
