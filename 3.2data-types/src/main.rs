// 数据类型
// 两类数据类型子集：标量（scalar）和复合（compound）。
fn main() {
    // 当多种类型均有可能时，必须增加类型注解
    // 比如第二章的 “比较猜测的数字和秘密数字” 使用 parse 将 String 转换为数字时
    let guess: u32 = "42".parse().expect("Not a number!");
    integer();
    tupe();
    array();
}

// Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类
// https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html#%E6%95%B0%E6%8D%AE%E7%B1%BB%E5%9E%8B

// 整型
fn integer() {
    let i: i32 = 0;
    let i = 98_222;
    let i = 0xff;
    let i = 0o77;
    let i = 0b1111_0000;
    let i = b'A';
    // let u: u32 = -1;
    // let i: i8 = 256;
    println!("{}", i);
}

// 浮点型
fn float() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}

fn bool_demo() {
    let t = true;

    let f: bool = false; // 显式指定类型注解
}

fn char_demo() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}

// 元组类型
fn tupe() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 可以使用模式匹配（pattern matching）来解构（destructure）元组值
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
    // 除了使用模式匹配解构外，也可以使用点号（.）后跟值的索引来直接访问它们
    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;
    println!("The value of five_hundred is: {}", five_hundred);
}

// 数组类型
// 1. 与元组不同，数组中的每个元素的类型必须相同。
// 2. 因为 Rust 中的数组是固定长度的：一旦声明，它们的长度不能增长或缩小。
// 3. 当你想要在栈（stack）而不是在堆（heap）上为数据分配空间（第四章将讨论栈与堆的更多内容），或者是想要确保总是有固定数量的元素时，数组非常有用。
fn array() {
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; //let a = [3, 3, 3, 3, 3];

    array_index();
}
// 编译并没有产生任何错误，不过程序会出现一个 运行时（runtime）错误并且不会成功退出。当尝试用索引访问一个元素时，Rust 会检查指定的索引是否小于数组的长度。如果索引超出了数组长度，Rust 会 panic，
fn array_index() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);
}
