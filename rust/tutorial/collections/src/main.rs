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

    // Test the entry method
    scores.entry(String::from("Blue")).or_insert(70);

    for (key, value) in scores {
        println!("Score for team {}: {}", key, value);
    }

    let s = "Did you hear the story of the man? Did you hear it?";
    let mut map = HashMap::new();
    for word in s.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn max<T>(list: &[T]) -> T
    where T: PartialOrd + Copy {
    let mut max = list[0];
    for &item in list.iter() {
        if item > max {
            max = item;
        }
    }

    max
}

fn sum(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list {
        sum += val;
    }
    sum
}

fn average(list: &[i32]) -> f32 {
    let mut sum = 0;
    for &val in list {
        sum += val;
    }
    let mean: f32 = sum as f32 / list.len() as f32;
    mean
}

fn main() {
    strings();
    hash();

    let v = vec![1, 6, 127, 4, 33, 42, 27, 8009];
    println!("v: {:?}", v);
    println!("Max of v: {}", max(&v));
    println!("Sum of v: {}", sum(&v));
    println!("Average of v: {}", average(&v));
}
