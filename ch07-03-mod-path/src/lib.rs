#[allow(unused)]
mod front_of_house {
    pub mod hosting {
        // 使用 super 起始的相对路径
        pub fn add_to_waitlist() {
            println!("add_to_waitlist");
            super::super_fn();
        }
    }
    fn super_fn() {
        println!("super fn");
    }
}

mod black_of_house {
    // 与之相反，如果我们将枚举设为公有，则它的所有成员都将变为公有
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    //
}
