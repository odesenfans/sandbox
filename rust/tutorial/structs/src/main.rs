struct User {
    username: String,
    email: String,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let u = build_user(String::from("odesenfans"), String::from("desenfans.olivier@gmail.com"));
    println!("{} ({})", u.username, u.email);

    let rect = Rectangle {width: 40, height: 100};
    println!("The area of the rectangle is {} pixels", area(&rect));
    println!("rect is {:?}", rect);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
    }
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
