// Debug 是一个 trait，它允许我们以一种对开发者有帮助的方式打印结构体，以便当我们调试代码时能看到它的值。
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 实现方法
// impl 是 implementation 的缩写
// 使用 &self 来替代 rectangle: &Rectangle
/*
   注意仍然需要在 self 前面加上 &，就像 &Rectangle 一样。方法可以选择获取 self 的所有权，
   或者像我们这里一样不可变地借用 self，或者可变地借用 self，就跟其他参数一样。
*/
impl Rectangle {
    // 借用self
    fn area(&self) -> u32 {
        // self.width = 100; //因为借用所以不可以写入
        self.width * self.height
    }
    // 引用self
    fn area2(self) -> u32 {
        self.width * self.height //因为引用,所以用完就消失了
    }

    fn area3(&mut self) -> u32 {
        self.width = 50; //可变引用
        self.width * self.height
    }
}

fn main() {
    let mut react1 = Rectangle {
        width: 30,
        height: 50,
    };
    //Debug 打印
    println!("{:?}", react1);
    //Debug Pretty 打印
    println!("{:#?}", react1);
    println!(
        "The area of the rectangle is {} square pixels.",
        react1.area()
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        react1.area3()
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        react1.area2()
    );

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     react1.area()
    // );
}

fn area(react: &Rectangle) -> u32 {
    react.width * react.height
}

// 为什么使用method这个概念

// 使用方法替代函数，除了可使用方法语法和不需要在每个函数签名中重复 self 的类型之外，其主要好处在于组织性。
// 我们将某个类型实例能做的所有事情都一起放入 impl 块中，而不是让将来的用户在我们的库中到处寻找 Rectangle 的功能。
