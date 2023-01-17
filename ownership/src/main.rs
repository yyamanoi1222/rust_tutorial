fn main() {
    let x = String::from("x");
    let y = x;
    println!("Hello, world! {}", y);

    take_ownership(y);

    let x = String::from("x");
    let x = take_and_gives_back(x);

    println!("Hello, world! {}", x);

    let x = String::from("test");
    let len = calculate_length(&x);

    println!("calc length {}", len);
    println!("x is {}", x);

    let mut x = String::from("test");
    change(&mut x);

    println!("x is {}", x);
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn take_and_gives_back(s: String) -> String {
    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", test");
}
