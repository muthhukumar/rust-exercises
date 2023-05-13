pub mod book;
pub mod main;

pub struct Library {
    books: Vec<book::Book>,
}

impl Library {
    pub fn add(&mut self, new_book: book::Book) {
        self.books.push(new_book);
    }
    pub fn new() -> Library {
        Library { books: Vec::new() }
    }

    // TODO - implement this
    pub fn search_by_title(&self, query: String) -> Vec<book::Book> {
        println!("Searching for books that matches the query: {}", query);

        Vec::new()
    }
    // TODO - implement this
    pub fn search_by_author(&self, query: String) -> Vec<book::Book> {
        println!("Searching for books that matches the author: {}", query);

        Vec::new()
    }

    pub fn display_all_books(&self) {
        println!("Displaying all the books...");
        for book in &self.books {
            book.display_info();
            println!();
        }
    }
}
