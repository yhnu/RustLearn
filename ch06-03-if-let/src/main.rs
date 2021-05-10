// 使用if let简化控制
#![allow(unused)]
fn main() {
    let some_u8_value = Some(3);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // if let 获取通过等号分隔的一个模式和一个表达式。它的工作方式与 match 相同，这里的表达式对应 match 而模式则对应第一个分支。
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
