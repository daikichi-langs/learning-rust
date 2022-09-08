trait Fruits {
    fn price(&self) -> u32;
}

struct Apple;
impl Fruits for Apple {
    fn price(&self) -> u32 {
        5
    }
}

struct Banana;
impl Fruits for Banana {
    fn price(&self) -> u32 {
        10
    }
}

trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

trait Message {
    fn message(&self) -> String {
        String::from("message")
    }
}
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}
impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{} by {} {}", self.headline, self.author, self.location)
    // }
}
impl Message for NewsArticle {}

struct Tweet {
    username: String,
    content: String,
    replay: bool,
    retweet: u32,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn run() {
    let apple = Apple {};
    let banana = Banana {};

    get_price(apple);
    get_price(banana);

    let tweet = Tweet {
        username: String::from("foo"),
        content: String::from("of corse, foo began rust. i heard."),
        replay: false,
        retweet: 0,
    };
    println!("{}", tweet.summarize());

    let newsarticle = NewsArticle {
        headline: String::from("Foo began Rust!"),
        location: String::from("Paris"),
        author: String::from("bar"),
        content: String::from("Foo began Rust! Foo began Rust! Foo began Rust!"),
    };
    println!("{}", newsarticle.summarize());

    notify(&tweet);
    notify(&newsarticle);
    notify_(&newsarticle);
    // notify_(&tweet);
}
fn get_price<T: Fruits>(fruits: T) {
    println!("price: {}.", fruits.price());
}
fn notify(item: &impl Summary) {
    println!("[Breaking News!] {}", item.summarize());
}
fn notify_(item: &(impl Summary + Message)) {
    println!("[Breaking news] {}", item.summarize());
    println!("[Message] {}", item.message());
}
