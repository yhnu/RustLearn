// Rust 中一个实现消息传递并发的主要工具是 通道（channel），Rust 标准库提供了其实现的编程概念。
// 你可以将其想象为一个水流的通道，比如河流或小溪。
// 如果你将诸如橡皮鸭或小船之类的东西放入其中，它们会顺流而下到达下游。
use std::sync::mpsc;
use std::thread;

// 通过克隆发送者来创建多个生产者
fn main() {
    let (tx, rx) = mpsc::channel();

    //  克隆多个生产者
    let tx2 = tx.clone();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    thread::spawn(move || {
        let val = String::from("hi");
        tx2.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
