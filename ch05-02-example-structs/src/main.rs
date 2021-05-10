// Debug 是一个 trait，它允许我们以一种对开发者有帮助的方式打印结构体，以便当我们调试代码时能看到它的值。
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let react1 = Rectangle {
        width: 30,
        height: 50,
    };
    //Debug 打印
    println!("{:?}", react1);
    //Debug Pretty 打印
    println!("{:#?}", react1);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&react1) //引用传输
    );
}

fn area(react: &Rectangle) -> u32 {
    react.width * react.height
}
