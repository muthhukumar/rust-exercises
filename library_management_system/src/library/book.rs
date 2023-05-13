pub struct Book {
    title: String,
    author: String,
}

impl Book {
    pub fn new(title: String, author: String) -> Book {
        Book { title, author }
    }

    pub fn display_info(&self) {
        println!("Book information");
        println!("Title: {}", self.title);
        println!("Author: {}", self.author);
    }
}
