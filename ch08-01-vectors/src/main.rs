#![allow(unused)]
fn main() {
    // 新建 vector, 并注解数据类型
    let v: Vec<i32> = Vec::new();
    // 自动推算类型信息
    let mut v = vec![1, 2, 3];

    // 更新 vector
    v.push(4);

    println!("{:?}", v);

    // 读取 vector 的元素
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    v.remove(2);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    /*
    [20210511133156](https://cdn.jsdelivr.net/gh/yhnu/PicBed/images20210511133156.png)
    index out of bound
    */
    // let v = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);

    // 借用检查器将会执行所有权和借用规则（第四章讲到）来确保 vector 内容的这个引用和任何其他引用保持有效
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // v.push(6); // 这里编译器会帮你做检查

    println!("The first element is: {}", first);

    ///
    /// 遍历 vector 中的元素
    ///
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    ///
    /// 使用枚举来储存多种类型
    ///
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    // 丢弃 vector 时也会丢弃其所有元素
}

//如果在编写程序时不能确切无遗地知道运行时会储存进 vector 的所有类型，枚举技术就行不通了。相反，你可以使用 trait 对象，第十七章会讲到它。
