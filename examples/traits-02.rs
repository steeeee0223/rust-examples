use rust_examples::summary::{news_article::NewsArticle, notify};

fn main() {
    println!("🎯 Traits Example - 2. Notify");

    let news_article: NewsArticle = NewsArticle {
        headline: String::from("Head News!"),
        location: String::from("US"),
        author: String::from("annonemous"),
        content: String::from("This is an exciting news..."),
    };

    notify(&news_article);
}
