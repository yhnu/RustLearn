// Rust 代码中最常见的用来控制执行流的结构是 if 表达式和循环
fn fn_if_else() {
    let number = 3;

    // 1. 不需要括号
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    //使用过多的 else if 表达式会使代码显得杂乱无章，所以如果有多于一个 else if 表达式，最好重构代码。
    //为此，第六章会介绍一个强大的 Rust 分支结构（branching construct），叫做 match。

    // 2. 在 let 语句中使用 if
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}

// 使用循环重复执行
// Rust 有三种循环：loop、while 和 for。我们每一个都试试。

fn fn_loop() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    loop {
        println!("again!");
    }
}

fn fn_while() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }
}

fn fn_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4) {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn main() {
    fn_for();
}
