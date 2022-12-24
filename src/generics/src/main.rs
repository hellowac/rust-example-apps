use crate::aggregator::{Summary, Tweet};
use crate::back::main2;

pub mod aggregator;
pub mod back;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    main2();
}
