pub fn run() {
    let mut book = String::from("The Rust Programming Language");

    read_book(&book); 

    annotate_book(&mut book, " - What a nice book!"); 

    read_book(&book); 

    println!("I am still got a book: {}", book);
    }

    fn read_book(book: &String) {
        println!("Reading: {}", book);
    }

    fn annotate_book(book: &mut String, note: &str) {
        book.push_str(note);
        println!("Added annotate to book");
    }


    /*
    Output:
    Đang đọc: The Rust Programming Language
    Đã add ghi chú vào book
    Đang đọc: The Rust Programming Language - Sách hay!
    Tôi vẫn sở hữu book: The Rust Programming Language - Sách hay!
    */
    /*
    book.push_str(note); thay đổi nội dung của book.
    */