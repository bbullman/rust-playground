/*
Modify the look_at_apples function below so that this program produces the output:

Count: normal consecutive apples
Count: good number of apples
Count: lots of apples
Count: over nine thousand
*/

fn look_at_apples(_apples: i32) {
  if _apples < 100 {
    println!("Count: normal consecutive apples");
  } else if _apples >= 100 && _apples < 1000 {
    println!("Count: good number of apples");
  } else if _apples >= 1000 && _apples < 9001 {
    println!("Count: lots of apples");
  } else {
    println!("Count: over nine thousand");
  }
}

fn main() {
  look_at_apples(9000)
}
