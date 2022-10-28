// Traits: Defining Shared Behavior
// Note: Traits are similar to a feature often called interfaces in other languages, 
// although with some differences.

// Defining a Trait
// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// Defining a Trait
// use traits::{Summary, Tweet, NewsArticle};

// fn main() {
//     let tweet = Tweet {
//         username: String::from("Witold"),
//         content: String::from(
//             "bro those hopgoblins are cringe",
//         ),
//         reply: false,
//         retweet: false,
//     };

//     let news_article = NewsArticle {
//         headline: String::from("THE COMMAND ATTACKS"),
//         location: String::from("Haless"),
//         author: String::from("One Xia Government"),
//         content: String::from("Hopgoblin armies started marching for conquest in all direction"),
//     };

//     println!("Tweet summary: {}", tweet.summarize());
//     println!("News article summary: {}", news_article.summarize());
// }


// Default Implementations
// use traits::{Summary, Tweet, NewsArticle};
// fn main() {
//     let tweet = Tweet {
//         username: String::from("Witold"),
//         content: String::from(
//             "bro those hopgoblins are cringe",
//         ),
//         reply: false,
//         retweet: false,
//     };

//     let news_article = NewsArticle {
//         headline: String::from("THE COMMAND ATTACKS"),
//         location: String::from("Haless"),
//         author: String::from("One Xia Government"),
//         content: String::from("Hopgoblin armies started marching for conquest in all direction"),
//     };

//     println!("Tweet summary: {}", tweet.summarize());
//     println!("News article summary: {}", news_article.summarize());
// }

// Default Implementations 2
// use traits::{Summary, Tweet};
// fn main() {
//     let tweet = Tweet {
//         username: String::from("Witold"),
//         content: String::from(
//             "bro those hopgoblins are cringe",
//         ),
//         reply: false,
//         retweet: false,
//     };

//     println!("Tweet summary: {}", tweet.summarize());
// }

