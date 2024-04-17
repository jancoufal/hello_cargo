use crate::back_of_house::say_breakfast;

mod front_of_house;
mod back_of_house;

fn main() {
    front_of_house::greetings("enter");
    front_of_house::hosting::add_to_waitlist();
    front_of_house::serving::take_order();
    front_of_house::serving::take_payment_and_eat_again();
    front_of_house::serving::take_payment_and_eat_again2();

    let breakfast = back_of_house::breakfast::Breakfast::summer("White bread");
    println!("{}", breakfast.to_string());

    let mut breakfast = back_of_house::Breakfast::summer("Rye");
    println!("{}", breakfast.to_string());
    breakfast.toast = String::from("Wheat");
    println!("{}", breakfast.to_string());

    say_breakfast();

    use_of_use();
}

fn use_of_use() {
    println!("use of use");
    use front_of_house::serving;
    serving::take_order();
    serving::take_payment_and_eat_again();
}
