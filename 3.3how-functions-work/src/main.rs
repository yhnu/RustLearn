// 函数
// Rust 代码中的函数和变量名使用 snake case 规范风格。在 snake case 中，所有字母都是小写并使用下划线分隔单词。
fn main() {
    println!("Hello, world!");

    another_function();
    main2();
    statements();
}

fn another_function() {
    println!("Another function.");
}

// 函数也可以被定义为拥有 参数（parameters），参数是特殊变量，是函数签名的一部分。
fn main2() {
    another_function2(5);
}

fn another_function2(x: i32) {
    println!("The value of x is: {}", x);
}

// 语句（Statements）是执行一些操作但不返回值的指令。表达式（Expressions）计算并产生一个值
fn statements() {
    let y = 6; //表达式
               // let x = (let y = 6); //let y = 6 语句并不返回值，所以没有可以绑定到 x 上的值
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
    // 表达式可以是语句的一部分：在示例 3-1 中，
    // 语句 let y = 6; 中的 6 是一个表达式，它计算出的值是 6。
    // 函数调用是一个表达式。
    // 宏调用是一个表达式。
    // 我们用来创建新作用域的大括号（代码块），{}，也是一个表达式
}

// 1. 函数可以向调用它的代码返回值。我们并不对返回值命名，但要在箭头（->）后声明它的类型
// 2. 使用 return 关键字和指定值，可从函数中提前返回；
// 3. 但大部分函数隐式的返回最后的表达式。这是一个有返回值的函数的例子：
fn five() -> i32 {
    5
}
