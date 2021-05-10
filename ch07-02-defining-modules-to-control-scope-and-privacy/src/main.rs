// 如允许你命名项的 路径（paths）
// 用来将路径引入作用域的 use 关键字；
// 以及使项变为公有的 pub 关键字
// as 关键字、
// 外部包
// glob 运算符

// 之所以这样叫它们是因为这两个文件的内容都分别在 crate 模块结构的根组成了一个名为 crate 的模块，该结构被称为 模块树（module tree）。

// 注意，整个模块树都植根于名为 crate 的隐式模块下。
fn main() {
    println!("Hello, world!");
}
