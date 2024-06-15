/*
 * Write a function to calculate the average of the scores passed as an argument of type Vec.
 */

#[allow(unused_variables, unused_mut)]
fn average(scores: &Vec<usize>) -> usize {
    let mut sum = 0;
    for i in scores {
        sum += i;
    }
    sum / scores.len()
}

fn main() {
    println!("{}", average(&vec![1,2,3,4,5,6,7]));
}
