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

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let u = build_user(String::from("odesenfans"), String::from("desenfans.olivier@gmail.com"));
    println!("{} ({})", u.username, u.email);

    let rect1 = Rectangle {width: 40, height: 100};
    let rect2 = Rectangle {width: 20, height: 59};
    println!("The area of rect1 is {} pixels", rect1.area());
    println!("rect1 is {:?}", rect1);
    println!("rect2 is {:?}", rect2);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
    }
}
