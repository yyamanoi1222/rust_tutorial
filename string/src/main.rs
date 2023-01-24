fn main() {
    let mut s = String::new();


    let data = "initial contents";

    let s = data.to_string();

    let s = "initial contents".to_string();

    let mut s = String::from("initial contents");
    s.push_str("test");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    let s1 = String::from("hello");
    let h = s1[0];
}
