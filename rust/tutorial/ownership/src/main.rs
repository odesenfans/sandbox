fn main() {
    let mut s = String::from("Hello");

    let s2 = &mut s;

    change(s2);
    println!("{}", s2);
}

fn change(s: &mut String) {
    s.push_str(", world");
}
