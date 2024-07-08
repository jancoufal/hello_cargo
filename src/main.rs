fn main() {
    static X: i32 = 22;
    let a: &'static i32 = &X;
    let b: &'static i32 = &X;

    println!("X: {}", X);
    println!("a: {}", a);
    println!("b: {}", b);
}

