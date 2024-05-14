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

    tests();
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

fn tests() {
    task_1();
    task_2();
    task_2_chat_gpt();
}

fn task_1() {
    // Given a list of integers, use a vector and return the median
    // (when sorted, the value in the middle position) and mode (the
    // value that occurs most often; a hash map will be helpful here)
    // of the list.
    let randoms = vec![11,33,55,77,99,22,44,66,88];
    let mut sorted = randoms.clone();
    sorted.sort();

    let median_index = sorted.len() / 2;
    let median = sorted.get(median_index);
    println!("median of {:?} is {:?}", randoms, median);
}

fn task_2() {
    let input = "Convert strings to pig latin. The first consonant of \
    each word is moved to the end of the word and “ay” is added, so “first” \
    becomes “irst-fay.” Words that start with a vowel have “hay” added to \
    the end instead (“apple” becomes “apple-hay”). Keep in mind the details \
    about UTF-8 encoding!".to_string();

    let vowels = "aeiouyAEIOUY".to_string();

    input.split_whitespace().for_each(|w| {
        match w.chars().nth(0) {
            None => {}
            Some(c) => {
                if vowels.contains(c) {
                    // println!("Starts with vowel '{}':", c);
                    print!("{w}-hay");
                }
                else if c.is_alphabetic() {
                    // println!("Starts with consonant '{}'", c);
                    print!("{}-{c}ay", &w[1..]);
                }
                else {
                    // println!("Starts with unknown '{}'", c);
                    print!("{w}");
                }
            }
        }
        // println!("...{:?} [{:?}]", w, w.chars().nth(0));
        print!(" ");
    });

    // println!("{input}");
    println!();
}

fn task_2_chat_gpt() {
    let input = "Convert strings to pig latin. The first consonant of \
    each word is moved to the end of the word and “ay” is added, so “first” \
    becomes “irst-fay.” Words that start with a vowel have “hay” added to \
    the end instead (“apple” becomes “apple-hay”). Keep in mind the details \
    about UTF-8 encoding!";

    let vowels = "aeiouyAEIOUY";

    input.split_whitespace().for_each(|w| {
        if let Some(c) = w.chars().next() {
            if vowels.contains(c) {
                print!("{}-hay", w);
            } else if c.is_alphabetic() {
                print!("{}-{}ay", &w[1..], c);
            } else {
                print!("{}", w);
            }
        }
        print!(" ");
    });
    println!();
}

