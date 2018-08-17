use std::env;

fn fibonacci(n:u32) -> u32 {
    let mut x = 0;
    let mut y = 1;
    let mut tmp: u32;

    for _ in 0..n {
        tmp = x + y;
        x = y;
        y = tmp;
    }

    x
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let x = args[1].parse::<u32>().unwrap();
    println!("Arg: {}", x);
    println!("Fibonacci number of {}: {}", x, fibonacci(x));
}
