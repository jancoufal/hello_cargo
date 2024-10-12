#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    ChangeBgColor(Color),
}

fn main() {
    println!("test #1 - Matching literals");
    {
        let x = 1;

        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    println!("test #2 - Matching Named Variables");
    {
        let x = Some(5);
        let y = 10;

        match x {
            Some(5) | Some(50) => println!("Got 5 or 50"),
            Some(y) => println!("Matched, y = {y}"),
            _ => println!("Default case, x = {x:?}"),
        }

        println!("at the end: x = {x:?}, y = {y}");
    }

    println!("test #3 - Multiple Patterns");
    {
        let x = 1;

        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    println!("test #4a - Matching Ranges of Values with ..=");
    {
        let x = 5;

        match x {
            1..=5 => println!("one through five"),
            _ => println!("something else"),
        }
    }

    println!("test #4b - Matching Ranges of Values with ..=");
    {
        let x = 'c';

        match x {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }

    println!("test #5a - Destructuring Structs");
    {
        let p = Point { x: 0, y: 7 };
        println!("Point = {p:?}");

        let Point { x: a, y: b } = p;
        println!("a = {a}, b = {b}");
    }

    println!("test #5b - Destructuring Structs");
    {
        let p = Point { x: 3, y: 9 };
        println!("Point = {p:?}");

        let Point { x, y } = p;
        println!("x = {x}, y = {y}");
    }

    println!("test #5c - Destructuring Structs");
    {
        let p = Point { x: 0, y: 7 };
        println!("Point = {p:?}");

        match p {
            Point { x, y: 0 } => println!("On the x axis at {x}"),
            Point { x: 0, y } => println!("On the y axis at {y}"),
            Point { x, y } => {
                println!("On neither axis: ({x}, {y})");
            }
        }
    }

    println!("test #6 - Destructuring Enums");
    {
        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.");
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {x} and in the y direction {y}");
            }
            Message::Write(text) => {
                println!("Text message: {text}");
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {r}, green {g}, and blue {b}")
            }
            _ => {
                println!("Other variant");
            }
        }
    }

    println!("test #7 - Destructuring Nested Structs and Enums");
    {
        let msg = Message::ChangeBgColor(Color::Hsv(0, 160, 255));

        match msg {
            Message::ChangeBgColor(Color::Rgb(r, g, b)) => {
                println!("Change color to red {r}, green {g}, and blue {b}");
            }
            Message::ChangeBgColor(Color::Hsv(h, s, v)) => {
                println!("Change color to hue {h}, saturation {s}, value {v}")
            }
            _ => (),
        }
    }

    println!("test #8a - Ignoring Parts of a Value with a Nested _");
    {
        let mut setting_value = Some(5);
        let new_setting_value = Some(10);

        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value");
            }
            _ => {
                setting_value = new_setting_value;
            }
        }

        println!("setting is {setting_value:?}");
    }

    println!("test #8b - Ignoring Parts of a Value with a Nested _");
    {
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, _, third, _, fifth) => {
                println!("Some numbers: {first}, {third}, {fifth}")
            }
        }
    }

    println!("test #8c - Ignoring Parts of a Value with a Nested _");
    {
        let s = Some(String::from("Hello!"));

        if let Some(_s) = s {   // "s" could be used when "let Some(_)" is being used, because "_" do not bind the variable
            println!("found a string");
        }

        // println!("{s:?}");   // error, "s" moved to "_s"
    }

    println!("test #9a - Ignoring Remaining Parts of a Value with ..");
    {
        let origin = Point3D { x: 1, y: 2, z: 3 };
        println!("Point3D = {origin:?}");

        match origin {
            Point3D { x, .. } => println!("x is {x}"),
        }
    }

    println!("test #9b - Ignoring Remaining Parts of a Value with ..");
    {
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, .., last) => {
                println!("Some numbers: {first}, {last}");
            }
        }
    }

    println!("test #10 - Extra Conditionals with Match Guards");
    {
        let num = Some(4);

        match num {
            Some(x) if x % 2 == 0 => println!("The number {x} is even"),
            Some(x) => println!("The number {x} is odd"),
            None => (),
        }
    }

    println!("test #11 - @ Bindings");
    {
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello {
                id: id_variable @ 3..=7,
            } => println!("Found an id in range: {id_variable}"),
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range")
            }
            Message::Hello { id } => println!("Found some other id: {id}"),
        }
    }
}
