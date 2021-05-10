// Rust 则选择了第三种方式：通过所有权系统管理内存，编译器在编译时会根据一系列的规则进行检查。

// 栈（Stack）与堆（Heap）
// 访问堆上的数据比访问栈上的数据慢，因为必须通过指针来访问。现代处理器在内存中跳转越少就越快（缓存）。
// 继续类比，假设有一个服务员在餐厅里处理多个桌子的点菜。在一个桌子报完所有菜后再移动到下一个桌子是最有效率的。
// 从桌子 A 听一个菜，接着桌子 B 听一个菜，然后再桌子 A，然后再桌子 B 这样的流程会更加缓慢。
// 出于同样原因，处理器在处理的数据彼此较近的时候（比如在栈上）比较远的时候（比如可能在堆上）能更好的工作。在堆上分配大量的空间也可能消耗时间。

// 1.Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
// 2.值在任一时刻有且只有一个所有者。
// 3.当所有者（变量）离开作用域，这个值将被丢弃。
fn main() {
    println!("Hello, world!");
    string_scope();
    string_scope2();
    ownership_fun();
}

// String 类型(分配在堆上)
// Rust 有第二个字符串类型，String。这个类型被分配到堆上，所以能够存储在编译时未知大小的文本。
#[allow(unused)]
fn string_scope() {
    let s = String::from("hello");
    let mut s = String::from("hello"); // 从此处起，s 是有效的

    s.push_str(", world!"); // push_str() 在字符串后追加字面值

    println!("{}", s); // 将打印 `hello, world!`
} // s 不再有效
  // 注意：在 C++ 中，这种 item 在生命周期结束时释放资源的模式有时被称作
  // 资源获取即初始化（Resource Acquisition Is Initialization (RAII)）。
  // 如果你使用过 RAII 模式的话应该对 Rust 的 drop 函数并不陌生。

// 怎么解决内存拷贝问题
// 变量与数据交互的方式（一）：移动
#[allow(unused)]
fn string_scope2() {
    let s1 = String::from("hello");
    // 那么拷贝指针、长度和容量而不拷贝数据可能听起来像浅拷贝。
    // 不过因为 Rust 同时使第一个变量无效了，
    // 这个操作被称为 移动（move），而不是浅拷贝。
    let s2 = s1;

    // println!("{}, world!", s1);
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

// 设计哲学：Rust 永远也不会自动创建数据的 “深拷贝”。因此，任何 自动 的复制可以被认为对运行时性能影响较小。
#[allow(unused)]
fn example3() {
    let x = 5;
    // 原因是像整型这样的在编译时已知大小的类型被整个存储在栈上，所以拷贝其实际的值是快速的。这意味着没有理由在创建变量 y 后使 x 无效。
    let y = x;
    println!("x = {}, y = {}", x, y);
}

// 任何简单标量值的组合可以是 Copy 的，不需要分配内存或某种形式资源的类型是 Copy 的
// 所有整数类型，比如 u32。
// 布尔类型，bool，它的值是 true 和 false。
// 所有浮点数类型，比如 f64。
// 字符类型，char。
// 元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是。

// 所有权与函数
#[allow(unused)]
fn ownership_fun() {
    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ...
                        // ... 所以到这里不再有效
                        // s.push_str("world");
    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，所以在后面可继续使用 x
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

// 返回值与作用域
#[allow(unused)]
fn ownership_ret() {
    let s1 = gives_ownership(); // gives_ownership 将返回值
                                // 移给 s1

    let s2 = String::from("hello"); // s2 进入作用域

    let s3 = takes_and_gives_back(s2); // s2 被移动到
                                       // takes_and_gives_back 中,
                                       // 它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 移出作用域并被丢弃

fn gives_ownership() -> String {
    // gives_ownership 将返回值移动给
    // 调用它的函数

    let some_string = String::from("hello"); // some_string 进入作用域.

    some_string // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域

    a_string // 返回 a_string 并移出给调用的函数
}

// 设计哲学2: 变量的所有权总是遵循相同的模式：
// 将值赋给另一个变量时移动它。当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉，
// 除非数据被移动为另一个变量所有。

// 在每一个函数中都获取所有权并接着返回所有权有些啰嗦，设计哲学3使用references解决此问题
#[allow(unused)]
fn tupe_ret_owner_ship() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度
    (s, length)
}
