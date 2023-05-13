use library_management_system::library;

fn main() {
    let book = library::book::Book::new(
        String::from("The law of human nature"),
        String::from("Robert Greene"),
    );

    let book_2 = library::book::Book::new(String::from("Sapies"), String::from("Noah"));

    book.display_info();

    let mut library = library::Library::new();

    library.add(book);
    library.add(book_2);

    library.search_by_title(String::from("The Law of human nature"));

    println!();

    library.search_by_author(String::from("Robert Greene"));

    println!();

    library.display_all_books();
}
