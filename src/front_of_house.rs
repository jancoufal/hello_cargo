pub mod hosting;

pub fn greetings(s: &str) {
println!("front_of_house::greetings({s})");
}

pub mod serving {
    use crate::front_of_house::hosting::add_to_waitlist;

    pub fn take_order() {
        println!("front_of_house::serving::take_order()");
    }

    pub fn take_payment_and_eat_again() {
        println!("front_of_house::serving::take_payment_and_eat_again()");
        add_to_waitlist();
    }

    pub fn take_payment_and_eat_again2() {
        println!("front_of_house::serving::take_payment_and_eat_again2()");
        super::yes();  // relative path
        crate::front_of_house::yes();  // absolute path
    }
}

fn yes() {
    println!("yes")
}

