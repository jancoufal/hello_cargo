pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String;
    fn summarize2(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("'{}'", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: "horse_ebooks".to_string(),
        content: "of course, as you probably already know, people".to_string(),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new tweet: {}", tweet.summarize2());

    let article = NewsArticle {
        headline: "Penguins win the Stanley Cup Championship!".to_string(),
        location: "Pittsburgh, PA, USA".to_string(),
        author: "Iceburgh".to_string(),
        content: "The Pittsburgh Penguins once again are the best hockey team in the NHL.".to_string(),
    };

    println!("article: {}", article.summarize());
    println!("article: {}", article.summarize2());

    notify_1(&article);
    notify_2(&article);
    notify_3(&article);
}

pub fn notify_1(item: &impl Summary) {
    println!("notify_1 (a): {}", item.summarize());
    println!("notify_1 (b): {}", item.summarize2());
}

pub fn notify_2<T: Summary>(item: &T) {
    println!("notify_2 (a): {}", item.summarize());
    println!("notify_2 (b): {}", item.summarize2());
}

pub fn notify_3<T>(item: &T) where
    T: Summary,
{
    println!("notify_3 (a): {}", item.summarize());
    println!("notify_3 (b): {}", item.summarize2());
}

