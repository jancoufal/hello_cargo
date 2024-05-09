use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert("Blue".to_string(), 10);
    scores.insert("Yellow".to_string(), 50);

    let team_name = String::from("Red");
    let score = scores.get(&team_name).copied().unwrap_or(-1);

    dbg!(score);

    hm1(&scores);
    // dbg!(&scores);

    hm2(scores.clone());
    // dbg!(&scores);

    hm1(&scores);

    games();
}

fn hm1(m: &HashMap<String, i32>) {
    for (k, v) in m {
        println!("{k} = {v}")
    }
}
fn hm2(m: HashMap<String, i32>) { dbg!(m); }

fn games() {
    {
        println!("Overwriting the value");
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);
        println!("{:?}", scores);
    }
    {
        println!("Adding a Key and Value Only If a Key Isn’t Present");
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);
        println!("{:?}", scores);
    }
    {
        println!("Updating a Value Based on the Old Value");
        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{:?}", map);
    }
}