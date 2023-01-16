fn main() {
    let number = 3;

    if number < 3 {
        println!("under 3");
    } else if number < 5 {
        println!("under 5");
    }

    let condition = true;
    let a = if condition { 1 } else { 2 };

    println!("a is: {}", a);

    let a = [1, 2, 3, 4, 5];

    for el in a {
        println!("el is: {}", el);
    }

    for num in (1..4) {
        println!("num is: {}", num);
    }
}
