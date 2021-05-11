// OsString、OsStr、CString 和 CStr
// 这些字符串类型能够以不同的编码，或者内存表现形式上以不同的形式，来存储文本内容

#![allow(unused)]
fn main() {
    // 字符串新建
    // 直接堆上进行开辟
    let mut s = String::new();
    // 开辟在栈上
    let data = "initial contents";
    let s = data.to_string();
    println!("{:?}", s);

    let s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");

    // 使用 + 运算符或 format! 宏拼接字符串
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    // 使用函数签名 fn add(self, s: &str) -> String {
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = &s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{}-{}-{}", s1, s2, s3);

    // 索引字符串
    // 然而在 Rust 中，如果你尝试使用索引语法访问 String 的一部分，会出现一个错误。
    // [20210511143300](https://cdn.jsdelivr.net/gh/yhnu/PicBed/images20210511143300.png)
    let s1 = String::from("hello");
    // let h = s1[0];

    // 字节、标量值和字形簇（最接近人们眼中 字母 的概念）。
    // 最后一个 Rust 不允许使用索引获取 String 字符的原因是，索引操作预期总是需要常数时间 (O(1))。
    // 但是对于 String 不可能保证这样的性能，因为 Rust 必须从开头到索引位置遍历来确定有多少有效的字符。
    // 字符串 slice
    let hello = "Здравствуйте";

    let s = &hello[0..4];
}
