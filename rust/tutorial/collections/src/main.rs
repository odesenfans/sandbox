use std::collections::HashMap;

fn strings() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}

fn hash() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}

fn main() {
    strings();
    hash();
}
