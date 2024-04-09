use std::fmt;
use std::fmt::Formatter;

fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    let y = another_function(x);
    println!("y = {y:?}");
    println!("The value of x is: {x}");
    another_function(x);
    println!("The value of x is: {x}");

    let a = {
        let b = 8;
        println!("b = {b}");
        b + 1
    };

    println!("a = {a:?}");

    chapter_5_structs();
    chapter_6_enums_and_pattern_matching();
}

fn another_function(mut x: i32) -> i32 {
    x += 1;
    println!("The value of x is: {x}");
    x + 2
}

fn chapter_5_structs() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    impl fmt::Display for User {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{{ active: {}, username: \"{}\", email: \"{}\", sign_in_count: {}, }}",
                   self.active, self.username, self.email, self.sign_in_count
            )
        }
    }

    let u1 = User {
        active: false,
        username: String::from("John Doe"),
        email: "john.doe@world.com".to_string(),
        sign_in_count: 0,
    };

    println!("u1 = {u1}");

    fn create_user(username: &str, email: String) -> User {
        User {
            active: true,
            username: String::from(username),
            email,
            sign_in_count: 3,
        }
    }

    let u2 = create_user("username_in", "email_in".to_string());

    println!("u2 = {u2}");

    struct Color(u8, u8, u8);
    let red = Color(255, 20, 10);
    println!("red = ({}, {}, {})", red.0, red.1, red.2)
}

fn chapter_6_enums_and_pattern_matching() {
    #[derive(Debug)]
    enum IpAddrKind { V4, V6, }

    let ip_four = IpAddrKind::V4;
    let ip_six = IpAddrKind::V6;

    println!("{ip_four:?}");
    println!("{ip_six:?}");

    #[derive(Debug)]
    enum IpAddrMine {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let ip_home = IpAddrMine::V4(127, 0, 0, 1);
    let ip_loopback = IpAddrMine::V6("::1".to_string());

    println!("{ip_home:?}");
    println!("{ip_loopback:?}");

    fn print_ip(ip: &IpAddrMine) {
        match ip {
            IpAddrMine::V4(b0, b1, b2, b3) => {
                println!("v4={b0}.{b1}.{b2}.{b3}");
            },
            IpAddrMine::V6(addr) => {
                println!("v6={addr}")
            },
        }
    }

    print_ip(&ip_home);
    print_ip(&ip_loopback);


    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
        Arizona,
        Arkansas,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(UsState::Alabama) => {
                println!("State quarter from home sweet Alabama!");
                25
            },
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    dbg!(value_in_cents(Coin::Penny));
    dbg!(value_in_cents(Coin::Nickel));
    dbg!(value_in_cents(Coin::Dime));
    dbg!(value_in_cents(Coin::Quarter(UsState::Alabama)));
    dbg!(value_in_cents(Coin::Quarter(UsState::Alaska)));
    dbg!(value_in_cents(Coin::Quarter(UsState::Arizona)));
    dbg!(value_in_cents(Coin::Quarter(UsState::Arkansas)));
}