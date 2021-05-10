// 3.1 变量和可变性
fn main1() {
    let x = 5;
    println!("The value of x is: {}", x);
    //x = 6; //推动你以充分利用 Rust 提供的安全性和简单并发性来编写代码的众多方式之一
    //提高使用变量的代价来阻止你使用变量
    println!("The value of x is: {}", x);
}

fn main2() {
    let mut x = 5; //使用2个关键字来提高使用代价
                   //变量名之前加 mut 来使其可变。除了允许改变值之外，mut 向读者表明了其他代码将会改变这个变量值的意图
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

//1. 常量可以在任何作用域中声明，包括全局作用域，这在一个值需要被很多部分的代码用到时很有用。
//2. 常量只能被设置为常量表达式，而不能是函数调用的结果，或任何其他只能在运行时计算出的值。
const MAX_POINTS: u32 = 100_000;

fn main3() {
    //声明常量使用 const 关键字而不是 let，并且 必须 注明值的类型
    const MAX_POINTS: u32 = 100_000;
}

// 隐藏（Shadowing）
fn main4() {
    let x = 5;

    let x = x + 1; // 可以定义一个与之前变量同名的新变量，而新变量会 隐藏 之前的变量
                   // 隐藏与将变量标记为 mut 是有区别的。当不小心尝试对变量重新赋值时，如果没有使用 let 关键字，就会导致编译时错误。通过使用 let，我们可以用这个值进行一些计算，不过计算完之后变量仍然是不变的。
    let x = x * 2;

    println!("The value of x is: {}", x);

    // mut 与隐藏的另一个区别是，当再次使用 let 时，实际上创建了一个新变量，我们可以改变值的类型，且复用这个名字
    let spaces = "   ";
    let spaces = spaces.len();
}

fn main() {
    main4();
}
