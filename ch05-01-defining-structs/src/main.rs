struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[allow(unused)]
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // user1 不可变, 下面的语句不生效
    // user1.email = String::from("buutuud@gmail.com");

    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user2.email = String::from("buutuud@gmail.com");

    println!("Hello, world! {}", user1.email);
    println!("Hello, world! {}", user2.email);
    println!("Hello, world! {}", user2.username);

    let user3 = build_user(String::from("buutuud@mgial.com"), String::from("yhnu"));
    let user3 = build_user2(String::from("buutuud@mgial.com"), String::from("yhnu"));
    println!("Hello, world! {}", user3.email);
    build_user3();
    build_user3_simple();
    tuple_structs();
}

// 另外需要注意同其他任何表达式一样，我们可以在函数体的最后一个表达式中构造一个结构体的新实例，来隐式地返回这个实例。
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// 语法糖1
// 变量与字段同名时的字段初始化简写语法
fn build_user2(email: String, username: String) -> User {
    User {
        email, //名称相同可以简写
        username,
        active: true,
        sign_in_count: 1,
    }
}

// 语法糖2
// 使用结构体更新语法从其他实例创建实例
fn build_user3() -> User {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    user2
}

fn build_user3_simple() -> User {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    user2
}

// 简化3
// tuple structs
// 元组结构体有着结构体名称提供的含义，但没有具体的字段名，只有字段的类型。当你想给整个元组取一个名字，并使元组成为与其他元组不同的类型时，元组结构体是很有用的，
// 这时像常规结构体那样为每个字段命名就显得多余和形式化了。

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_structs() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    // let tup:(i32, i32, i32) = black;

    println!("{} {} {}", black.0, black.1, black.2);    
}


// unit-like structs

// 可以使结构体存储被其他对象拥有的数据的引用，不过这么做的话需要用上 生命周期（lifetimes）
// 如果你尝试在结构体中存储一个引用而不指定生命周期将是无效的