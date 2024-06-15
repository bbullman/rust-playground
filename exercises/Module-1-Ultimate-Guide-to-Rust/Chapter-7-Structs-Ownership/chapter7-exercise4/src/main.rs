/*
 * Fix the warnings in this code by changing the names around.A
 */

struct Point {
    x: i32,
    y: i32,
}

fn print_point(point: Point) {
    println!("x == {}, y == {}", point.x, point.y);
}

fn main() {
    print_point(Point { x: 3, y: -6 });
}
