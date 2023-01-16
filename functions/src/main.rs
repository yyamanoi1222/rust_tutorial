fn main() {
    println!("Hello, world!");
    another_func(1);
}

fn another_func(x: i32) {
    println!("another func: {}", x);
}

fn five() -> i32 {
    return 5
}

fn five2() -> i32 {
    5
}
