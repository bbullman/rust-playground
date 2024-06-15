/*
 * Write a modified sum function that takes a mutable reference for the destination of the sum from
 * low to high.
 */

#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) -> i32{
    println!("total {}, low {}, high {}", *total, low, high);
    let mut count: i32 = low;
    while count <= high {
        *total += count;
        println!("total {}", *total);
        count += 1;
    }
    *total
}

fn main() {
  let mut total: i32 = 0;
  println!("{}", sum(&mut total, 5, 100))
}
