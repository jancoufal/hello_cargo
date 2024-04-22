fn main() {
    let s = "🧻🤪👻🛀";

    dbg!(s);
    dbg!(s.len());
    dbg!(&s);
    dbg!(&s.len());
    dbg!(&s[..]);
    dbg!(&s[..4]);
    dbg!(&s[4..8]);
    dbg!(&s[8..]);
    // dbg!(&s[1..5]); // panic!

    for c in s.chars() {
        print!("{c}..")
    }
    println!();

    dbg!(&s.chars().count());
    dbg!(&s.chars().nth(3));
    dbg!(&s.chars().nth(4));
}
