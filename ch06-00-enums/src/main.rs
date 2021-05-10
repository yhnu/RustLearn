#![allow(unused)]

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    fn route(ip_type: IpAddrKind) {
        println!("{:#?}", ip_type);
    }

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("{:#?}", home);
    println!("{:#?}", loopback);

    let home = IpAddr2::V4(127, 0, 0, 1);
    let loopback = IpAddr2::V6(String::from("::1"));
    println!("{:#?}", home);
    println!("{:#?}", loopback);
    exmaple();
}

#[derive(Debug)]
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 更复杂的枚举, 每个成员都存储了不同数量和类型的值
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 为枚举定义方法
impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}

fn exmaple() {
    let m = Message::Write(String::from("hello"));
    println!("{:#?}", m);
}

// 标准库中的另一个非常常见且实用的枚举：Option。 为了解决空值的概念
// Option 枚举和其相对于空值的优势
// 1. Option 类型应用广泛因为它编码了一个非常普遍的场景，即一个值要么有值要么没值。
//    从类型系统的角度来表达这个概念就意味着编译器需要检查是否处理了所有应该处理的情况，这样就可以避免在其他编程语言中非常常见的 bug。

// Option<T> 枚举是如此有用以至于它甚至被包含在了 prelude 之中，你不需要将其显式引入作用域。
// 另外，它的成员也是如此，可以不需要 Option:: 前缀来直接使用 Some 和 None。即便如此 Option<T> 也仍是常规的枚举，Some(T) 和 None 仍是 Option<T> 的成员。

fn exmaple2() {
    // 1.你不需要将其显式引入作用域
    // 2.可以不需要 Option:: 前缀
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
}

fn exmaple3() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // 下面的表达式会报错
    // match 表达式就是这么一个处理枚举的控制流结构：它会根据枚举的成员运行不同的代码，这些代码可以使用匹配到的值中的数据。
    // let sum = x + y;
    // 总的来说，为了使用 Option<T> 值，需要编写处理每个成员的代码。你想要一些代码只当拥有 Some(T) 值时运行，允许这些代码使用其中的 T。
}
