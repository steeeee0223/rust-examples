use generics_traits::summary::Summary;
use generics_traits::tweet::Tweet;

fn main() {
    println!("🎯 Traits Example - 1. Summary");

    let tweet: Tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("💡 1 new tweet: {}", tweet.summarize());
}
