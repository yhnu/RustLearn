// 类似于 vector，哈希 map 是同质的：
// 所有的键必须是相同类型，值也必须都是相同类型。
#![allow(unused)]
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 另一个构建哈希 map 的方法是使用一个元组的 vector 的 collect 方法，其中每个元组包含一个键值对。
    // 可以使用 zip 方法来创建一个元组的 vector，其中 “Blue” 与 10 是一对，依此类推。接着就可以使用 collect 方法将这个元组 vector 转换成一个 HashMap

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // 访问哈希 map 中的值
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();

    // println!("{:?}", scores);
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    // or_insert
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    //
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
}

// HashMap 默认使用一种 “密码学安全的”（“cryptographically strong” ）1 哈希函数，它可以抵抗拒绝服务（Denial of Service, DoS）攻击。然而这并不是可用的最快的算法，不过为了更高的安全性值得付出一些性能的代价。如果性能监测显示此哈希函数非常慢，以致于你无法接受，你可以指定一个不同的 hasher 来切换为其它函数。
// hasher 是一个实现了 BuildHasher trait 的类型。第十章会讨论 trait 和如何实现它们。你并不需要从头开始实现你自己的 hasher；crates.io 有其他人分享的实现了许多常用哈希算法的 hasher 的库。
