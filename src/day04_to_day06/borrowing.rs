pub fn run() {
    // Create a mutable String to represent our book
    let mut book = String::from("The Rust Programming Language");

    // First, we read the book (immutable borrow)
    read_book(&book);

    // Then, we annotate the book (mutable borrow)
    annotate_book(&mut book, " - What a nice book!");

    // We can read the book again after annotation (immutable borrow)
    read_book(&book);

    // The book is still available in the main function
    println!("I still have the book: {}", book);
}

// This function takes an immutable reference to the book
fn read_book(book: &String) {
    println!("Reading: {}", book);
}

// This function takes a mutable reference to the book and a note to add
fn annotate_book(book: &mut String, note: &str) {
    book.push_str(note);
    println!("Added annotation to book");
}

/*
Expected Output:
Reading: The Rust Programming Language
Added annotation to book
Reading: The Rust Programming Language - What a nice book!
I still have the book: The Rust Programming Language - What a nice book!
*/

// Note: book.push_str(note) in annotate_book() modifies the content of the book.
// This is possible because we passed a mutable reference to the function.