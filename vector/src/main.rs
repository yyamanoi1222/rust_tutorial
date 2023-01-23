fn main() {
    let v: Vec<i32> = Vec::new();

    let v = vec![1,2,3];

    let mut v = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    {
        let v = vec![1,2,3];
    }

    let v = vec![1,2,3];

    let third: &i32 = &v[2];

    match v.get(2) {
        Some(thrid) => println!("The third value is {}", third),
        None => println!("no value"),
    }

    for i in &v {
        println!("i is {}", i)
    }

    enum Msg {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
    }

    let v = vec![
        Msg::Quit,
        Msg::Move{x: 1, y: 1},
        Msg::Write(String::from("")),
    ];
}
