// 注意：trait 类似于其他语言中的常被称为 接口（interfaces）的功能，虽然有一些不同。
// pub trait Summary {
//     fn summarize(&self) -> String;
// }

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

fn main() {
    println!("Hello, world!");
    let tweet = crate::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}

// 默认实现
// 这样当为某个特定类型实现 trait 时，可以选择保留或重载每个方法的默认行为。
pub struct Tweet2 {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// 不修改默认实现
impl Summary for Tweet2 {}

// trait作为参数
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait Bound 语法
pub fn notify2<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

// 不过如果你希望强制它们都是相同类型呢？这只有在使用 trait bound 时才有可能：
pub fn notify3<T: Summary>(item1: T, item2: T) {}

// 通过 + 指定多个 trait bound
// pub fn notify(item: impl Summary + Display) {
// pub fn notify<T: Summary + Display>(item: T) {

// 通过 where 简化 trait bound

// trait Display {}

fn some_function<T: Display + Clone, U: Clone>(t: T, u: U) -> i32 {}
//
fn some_function2<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone,
{
    40
}

// 返回实现了 trait 的类型
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
// impl Trait 允许你简单的指定函数返回一个 Iterator 而无需写出实际的冗长的类型。

// 使用 trait bound 有条件地实现方法
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
