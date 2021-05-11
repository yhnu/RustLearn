use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
// 注意与 Option 枚举一样，Result 枚举和其成员也被导入到了 prelude 中，所以就不需要在 match 分支中的 Ok 和 Err 之前指定 Result::

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("{:?}", error)
        }
    };

    // 失败时 panic 的简写：unwrap 和 expect
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // 还有另一个类似于 unwrap 的方法它还允许我们选择 panic! 的错误信息：expect
    // 使用 expect 而不是 unwrap 并提供一个好的错误信息可以表明你的意图并更易于追踪 panic 的根源
    // [20210511174903](https://cdn.jsdelivr.net/gh/yhnu/PicBed/images20210511174903.png)
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

// 这种传播错误的模式在 Rust 是如此的常见，以至于 Rust 提供了 ? 问号运算符来使其更易于处理。
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// ? 运算符可被用于返回 Result 的函数
