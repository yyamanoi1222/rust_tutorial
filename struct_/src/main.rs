struct User {
    username: String,
    email: String,
    sign_in_count: u64,
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

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    println!("Hello, world!");


    let u1 = User {
        username: String::from("user"),
        email: String::from("email"),
        sign_in_count: 1,
        active: true,
    };

    let mut u2 = User {
        username: String::from("user"),
        email: String::from("email"),
        sign_in_count: 1,
        active: true,
    };
    u2.email = String::from("u2 email");

    let r = Rectangle { width: 30, height: 30 };

    println!("r is {:?}", r);
    println!("area is {}", r.area());
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

fn build_from_user(u: User) -> User {
    User {
        ..u
    }
}
