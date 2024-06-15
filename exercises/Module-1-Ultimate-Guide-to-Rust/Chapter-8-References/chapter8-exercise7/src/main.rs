/* 
Let’s write a program that calculates the average of a series of provided values. The expected output is:

I don't have any values yet!
4
6
*/

struct RunningTotal {
    total: i32,
    count: i32,
}

fn new_running_total() -> RunningTotal {
    RunningTotal {
        total: 0,
        count: 0,
    }
}

fn print_average(rt: &RunningTotal) {
    if rt.total == 0 || rt.count == 0 {
        println!("I don't have any values yet!");
    } else {
        println!("{}", rt.total / rt.count);
    }
}

fn add_value(rt: &mut RunningTotal, value: i32) {
    rt.total += value;
    rt.count += 1;
}

fn main() {
    let mut rt = new_running_total();

    print_average(&rt);

    add_value(&mut rt, 5);
    add_value(&mut rt, 3);
    print_average(&rt);

    add_value(&mut rt, 10);
    print_average(&rt);
}

