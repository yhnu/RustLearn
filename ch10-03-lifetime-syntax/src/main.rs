// 知识点1:
//  Rust 需要我们使用泛型生命周期参数来注明他们的关系，这样就能确保运行时实际使用的引用绝对是有效的。

fn main() {
    example1();
}

// 优势:
// 生命周期避免了悬垂引用
// [20210512082503](https://cdn.jsdelivr.net/gh/yhnu/PicBed/images20210512082503.png)
fn example1() {
    let r;
    {
        let x = 5;
        r = &x; //这里借不了, x已经不存在了
    }
    println!("r: {}", r);
}

// 注意：示例 10-17、10-18 和 10-24 中声明了没有初始值的变量，所以这些变量存在于外部作用域。
// 这乍看之下好像和 Rust 不允许存在空值相冲突。
// 然而如果尝试在给它一个值之前使用这个变量，会出现一个编译时错误，这就说明了 Rust 确实不允许空值。

// 知识点2:
// 借用检查器。

// 接下来让我们聊聊在函数的上下文中参数和返回值的泛型生命周期。
// ![20210512091114](https://cdn.jsdelivr.net/gh/yhnu/PicBed/images20210512091114.png)

// 泛型生命周期参数需要声明在函数名和参数列表间的尖括号中
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// 记住通过在函数签名中指定生命周期参数时，我们并没有改变任何传入值或返回值的生命周期，而是指出任何不满足这个约束条件的值都将被借用检查器拒绝。
fn example12() {
    let s1 = String::from("Hello");
    let s2 = "World";

    println!("{}", longest(s1.as_str(), s2));
}

// 当具体的引用被传递给 longest 时，被 'a 所替代的具体生命周期是 x 的作用域与 y 的作用域相重叠的那一部分。换一种说法就是泛型生命周期 'a 的具体生命周期等同于 x 和 y 的生命周期中较小的那一个。
fn example13() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

fn example14() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str()); //string2作用域已经消失了
    }
    println!("The longest string is {}", result);
}

// 不过没有为参数 y 指定，因为 y 的生命周期与参数 x 和返回值的生命周期没有任何关系
fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// ![20210512092556](https://cdn.jsdelivr.net/gh/yhnu/PicBed/images20210512092556.png)
fn longest3<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}

// 结构体定义中的生命周期注解

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn example5() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    println!("{}", first_sentence);
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// 生命周期省略（Lifetime Elision）
// 总是重复地编写一模一样的生命周期注解。这些场景是可预测的并且遵循几个明确的模式。

// fn first_word<'a>(s: &'a str) -> &'a str {
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// 函数或方法的参数的生命周期被称为 输入生命周期（input lifetimes），而返回值的生命周期被称为 输出生命周期（output lifetimes）。
// 编译器采用三条规则来判断引用何时不需要明确的注解。第一条规则适用于输入生命周期，后两条规则适用于输出生命周期。如果编译器检查完这三条规则后仍然存在没有计算出生命周期的引用，编译器将会停止并生成错误。这些规则适用于 fn 定义，以及 impl 块。
// 第一条规则:
// 每一个是引用的参数都有它自己的生命周期参数。换句话说就是，有一个引用参数的函数有一个生命周期参数：fn foo<'a>(x: &'a i32)，
// 有两个引用参数的函数有两个不同的生命周期参数，fn foo<'a, 'b>(x: &'a i32, y: &'b i32)，依此类推

// 第二条规则:
// 如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：fn foo<'a>(x: &'a i32) -> &'a i32。

// 第三条规则:
// 如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，说明是个对象的方法(method)(译者注： 这里涉及rust的面向对象参见17章), 那么所有输出生命周期参数被赋予 self 的生命周期。第三条规则使得方法更容易读写，因为只需更少的符号。
// fn longest4(x: &str, y: &str) -> &str {
// fn longest4<'a, 'b>(x: &'a str, y: &'b str) -> &str { //推演不出来

// 方法定义中的生命周期注解
struct ImportantExcerpt2<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt2<'a> {
    fn level(&self) -> i32 {
        3
    }

    // 接着，因为其中一个参数是 &self，返回值类型被赋予了 &self 的生命周期
    // fn announce_and_return_part<'a,'b>(&'a self, announcement: &'b str) -> &'a str {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// 静态生命周期
fn example6() {
    let s: &'static str = "I have a static lifetime.";
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
