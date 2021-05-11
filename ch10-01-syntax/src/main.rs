fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &e in list {
        if e > largest {
            largest = e
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}

// 再一次简化

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &e in list.iter() {
        if e > largest {
            largest = e;
        }
    }
    largest
}

// 结构体定义范型
struct Point<T: Copy> {
    x: T,
    y: T,
}

fn main3() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}

struct Point2<T, U> {
    x: T,
    y: U,
}

fn main4() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}

// enum Option<T> {
//     Some(T),
//     None,
// }
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// 方法中定义范型
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// impl<f32> Point<f32> {
//     fn f32(&self) -> f32 {
//         self.x
//     }
// }

// Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率。
// 单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。
