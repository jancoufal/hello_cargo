mod front_of_house {

    pub fn greetings(s: &str) {
        println!("front_of_house::greetings({s})");
    }

    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("front_of_house::hosting::add_to_waitlist()");
            super::greetings("waiting list");
            reorder_waitlist();
        }

        fn reorder_waitlist() {
            println!("front_of_house::hosting::reorder_waitlist()");
        }
    }

    pub mod serving {
        pub fn take_order() {
            println!("front_of_house::serving::take_order()");
        }

        pub fn take_payment_and_eat_again() {
            println!("front_of_house::serving::take_payment_and_eat_again()");
            super::hosting::add_to_waitlist();
        }

        pub fn take_payment_and_eat_again2() {
            println!("front_of_house::serving::take_payment_and_eat_again2()");
            super::super::yes();  // relative path
            crate::yes();  // absolute path
        }
    }
}

mod back_of_house {

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: "peaches".to_string(),
            }
        }

        pub fn to_string(&self) -> String {
            String::from(format!("Breakfast({} with {});", self.toast, self.seasonal_fruit))
        }
    }
}

fn yes() {
    println!("yes")
}

fn main() {
    front_of_house::greetings("enter");
    front_of_house::hosting::add_to_waitlist();
    front_of_house::serving::take_order();
    front_of_house::serving::take_payment_and_eat_again();
    front_of_house::serving::take_payment_and_eat_again2();

    let mut breakfast = back_of_house::Breakfast::summer("Rye");
    println!("{}", breakfast.to_string());
    breakfast.toast = String::from("Wheat");
    println!("{}", breakfast.to_string());

    use_of_use();
}

fn use_of_use() {
    println!("use of use");
    use front_of_house::serving;
    serving::take_order();
    serving::take_payment_and_eat_again();
}