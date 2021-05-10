// 可以把 match 表达式想象成某种硬币分类器：硬币滑入有着不同大小孔洞的轨道，每一个硬币都会掉入符合它大小的孔洞。
// 同样地，值也会通过 match 的每一个模式，并且在遇到第一个 “符合” 的模式时，值会进入相关联的代码块并在执行中被使用。
#![allow(unused)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // 与switch很像，但是并不是switch
        // 这里它可以是任何类型的
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        //可以绑定匹配的模式的部分值
        Coin::Quarter(state) => {
            println!("{:#?}", state);
            15
        }
    }
}

// 可以绑定匹配的模式的部分值
#[derive(Debug)] // 这样可以可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

// 匹配 Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// 将 match 与枚举相结合在很多场景中都是有用的。你会在 Rust 代码中看到很多这样的模式：match 一个枚举，绑定其中的值到一个变量，接着根据其值执行代码。
// 这在一开始有点复杂，不过一旦习惯了，你会希望所有语言都拥有它！这一直是用户的最爱。

// _ 通配符
fn underline_match() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
// 然而，match 在只关心 一个 情况的场景中可能就有点啰嗦了。为此 Rust 提供了if let。

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    println!("Hello, world!");
}
