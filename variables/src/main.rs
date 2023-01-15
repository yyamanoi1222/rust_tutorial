fn main() {
    const P: u32 = 1;
    let mut x = 5;

    println!("the value of x is: {}", x);

    x = 6;

    println!("the value of x is: {}", x);

    {
        let x = x * 2;
        println!("the value of x in the inner scope is: {}", x);
    }

    println!("the value of x is: {}", x);

    let guess: i8 = "-1".parse().expect("not a number");
    println!("number is {}", guess);

    let c = 'c';

    let tup: (i32, i64) = (10, 20);

    let (x, y) = tup;
    let y = tup.1;

    println!("x is {}", x);
    println!("y is {}", y);

    let array = [1,2,3];
    let array1: [i32;5];
    let array2 = [1;5];
}
