fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let another_third: Option<&i32> = v.get(2);
    match another_third {
        None => println!("There is no third element."),
        Some(some_third) => println!("Another third element is {some_third}"),
    }

    v[2] *= 10;
    // drop(third);

    let third_2: &i32 = &v[2];
    println!("The third element is {third_2}");

    let another_third_2: Option<&i32> = v.get(2);
    match another_third_2 {
        None => println!("There is no third element."),
        Some(some_third) => println!("Another third element is {some_third}"),
    }

    // println!("first third is {third}");
    vec_games();
    vec_of_enum();
}

fn vec_games() {
    let mut v = vec![1, 2, 3, 4, 5];
    println!("{:?}", v);

    for i in &mut v {
        *i *= 10;
    }
    println!("{:?}", v);

    for i in &mut v {
        *i /= 5;
    }
    println!("{:?}", v);

    for i in &v {
        print!("{i}..")
    }
    println!("{:?}", v);
}

fn vec_of_enum() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for r in &row {
        print!("r = {{");
        match r {
            SpreadsheetCell::Int(i) => print!("Int={i:?}"),
            SpreadsheetCell::Float(f) => print!("Float={f:?}"),
            SpreadsheetCell::Text(t) => print!("Text={t:?}"),
        }
        println!("}}");
    }
}
