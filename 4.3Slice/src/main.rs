#[allow(unused)]

fn main() {
    println!("Hello, world!");
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);
    // 语法糖1
    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2];
    // 语法糖2
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];
    // 语法糖3
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];
}
