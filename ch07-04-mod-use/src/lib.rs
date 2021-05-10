mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 使用use关键字
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// 另一方面，使用 use 引入结构体、枚举和其他项时，习惯是指定它们的完整路径。示例 7-14 展示了将 HashMap 结构体引入二进制 crate 作用域的习惯用法。
use std::fmt::Result;

// 使用 as 关键字提供新的名称
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}

// 使用 pub use 重导出名称
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// 使用外部包
// 修改Cargo.toml

// 嵌套路径来消除大量的 use 行
use std::{cmp::Ordering, io};

// 可以在嵌套路径中使用 self
// use std::io;
// use std::io::Write;
use std::io::{self, Write};

// 通过 glob 运算符将所有的公有定义引入作用域
use std::collections::*;
