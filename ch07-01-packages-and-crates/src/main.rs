// 集装箱里面包含多个包裹
// 包（package） 是提供一系列功能的一个或者多个 crate。

// 规则：
// 包中所包含的内容由几条规则来确立。一个包中
// 至多 只能 包含一个库 crate(library crate)；
// 包中可以包含任意多个二进制 crate(binary crate)；
// 包中至少包含一个 crate，无论是库的还是二进制的。
fn main() {
    println!("Hello, world!");
}
