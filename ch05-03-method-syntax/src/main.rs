#![allow(unused)]

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
    // 带有更多参数的方法 示例
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // 关联函数 示例
    // 因为它们与结构体相关联。它们仍是函数而不是方法，因为它们并不作用于一个结构体的实例
    // associated functions
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// 结构体允许 多个 impl 块

impl Rectangle {
    fn say(&self) {
        println!("widtd={}", self.width);
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

    // 使用结构体名和 :: 语法来调用这个关联函数：比如
    let sq = Rectangle::square(3);
    // 这个方法位于结构体的命名空间中：:: 语法用于关联函数和模块创建的命名空间。第七章会讲到模块。
}

fn area(react: &Rectangle) -> u32 {
    react.width * react.height
}

// 为什么使用method这个概念

// 使用方法替代函数，除了可使用方法语法和不需要在每个函数签名中重复 self 的类型之外，其主要好处在于组织性。
// 我们将某个类型实例能做的所有事情都一起放入 impl 块中，而不是让将来的用户在我们的库中到处寻找 Rectangle 的功能。

fn main2() {
    #[derive(Debug, Copy, Clone)]
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        fn distance(&self, other: &Point) -> f64 {
            let x_squared = f64::powi(other.x - self.x, 2);
            let y_squared = f64::powi(other.y - self.y, 2);

            f64::sqrt(x_squared + y_squared)
        }
    }
    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 5.0, y: 6.5 };
    p1.distance(&p2);
    (&p1).distance(&p2);
}

// 结构体让你可以创建出在你的领域中有意义的自定义类型。
// 通过结构体，我们可以将相关联的数据片段联系起来并命名它们，这样可以使得代码更加清晰。
// 方法允许为结构体实例指定行为，而关联函数将特定功能置于结构体的命名空间中并且无需一个实例。

// 但结构体并不是创建自定义类型的唯一方法：让我们转向 Rust 的枚举功能，为你的工具箱再添一个工具。
