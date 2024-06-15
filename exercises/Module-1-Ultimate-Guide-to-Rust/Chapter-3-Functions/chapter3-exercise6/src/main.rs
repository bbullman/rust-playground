/*
  Using only two println! macro calls, write a program that produces the following output:

  I'm counting to 10. Currently at: 1.
  I'm counting to 10. Currently at: 2.
  I'm counting to 10. Currently at: 3.
  I'm counting to 10. Currently at: 4.
  I'm counting to 10. Currently at: 5.
  I'm counting to 10. Currently at: 6.
  I'm counting to 10. Currently at: 7.
  I'm counting to 10. Currently at: 8.
  I'm counting to 10. Currently at: 9.
  Bored now!
*/

fn main() {
  /* No loops yet in the course! So don't use one unless you want to. */
    say(10, 1);
    say(10, 2);
    say(10, 3);
    say(10, 4);
    say(10, 5);
    say(10, 6);
    say(10, 7);
    say(10, 8);
    say(10, 9);
    println!("Bored now!")
}

fn say(a: i32, b: i32) -> () {
    println!("I'm counting to {}. Currently at: {}.", a, b)
}
