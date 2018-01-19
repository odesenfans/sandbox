fn main() {
    println!("Hello, world!");
    another_function(27);
    println!("The answer is {}", the_answer_to_life_and_everything());

    countdown();
}

fn another_function(x: i32) {
    println!("What what: {}!", x);
}

fn the_answer_to_life_and_everything() -> i32 {
    42
}

fn countdown() {
    for i in (0..10).rev() {
        println!("{}", i);
    }

    println!("Happy New Year!");
}
