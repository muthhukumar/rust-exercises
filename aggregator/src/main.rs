use aggregator::{Summary, Tweet};

fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

fn notify_gen<T>(item: T)
where
    T: Summary,
{
    println!("{}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        content: "This is a content".to_string(),
        username: "muthukumar".to_string(),
        reply: false,
        retweet: false,
    };

    tweet.summarize()

    println!("tweet summary {}", tweet.summarize());
}
