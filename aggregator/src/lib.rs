pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}:{}", self.username, self.content)
    }
}

pub trait Notify {
    fn summarized(&self) {}
}

impl<T: Summary> Notify for T {
    fn summarized(&self) {
        println!("summarized")
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait CanFly {
    fn fly(&self);
}

pub trait Wing {
    fn swing 
}

struct Bird;

impl CanFly for Bird {
    fn fly(&self) {
        println!("The bird is flying.");
    }
}
