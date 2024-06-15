/*
 We originally wanted to be able to print the price for both the original and increased fruit amounts. In order to make that work, we need to be able to return both the original Fruit value and the price from the price_fruit function. Fix the program below to make it work. The expected output is:

I've got 10 apples and 5 bananas
Original price: 140
I can make 340 cents for more fruit

I’ve added FIXME comments in all places that need to be modified. The compiler can also be of great help in figuring out how to fix things.
*/

struct Fruit {
    apples: i32,
    bananas: i32,
}

struct FruitAndPrice {
    fruit: Fruit,
    price: i32,
}

fn count_fruit(fruit: Fruit) -> Fruit {
    println!(
        "I've got {} apples and {} bananas",
        fruit.apples, fruit.bananas
    );
    fruit
}

fn price_fruit(fruit: Fruit) -> FruitAndPrice {
    let price = fruit.apples * 8 + fruit.bananas * 12;
    FruitAndPrice {
        fruit,
        price,
    }
}


fn increase_fruit(mut fruit: Fruit) -> Fruit {
    fruit.apples *= 2;
    fruit.bananas *= 3;
    fruit
}

fn main() {
    let fruit = Fruit {
        apples: 10,
        bananas: 5,
    };

    let fruit_and_price = price_fruit(fruit);
    println!("Original price: {}", fruit_and_price.price);

    let fruit = increase_fruit(fruit_and_price.fruit);
    let fruit_and_price = price_fruit(fruit);
    println!("I can make {} cents for more fruit", fruit_and_price.price);

    let fruit = increase_fruit(fruit_and_price.fruit);
    let fruit_and_price = price_fruit(fruit);
    println!("I can make {} cents for even more fruit", fruit_and_price.price);
}

// Exercise 3: Modify the solution above to find out what the result would be if we increased the amount of fruit a second time.
