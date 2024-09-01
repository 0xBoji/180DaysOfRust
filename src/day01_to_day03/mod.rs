pub mod print;
pub mod strings;
pub mod functions;
pub mod types;
pub mod arrays;
pub mod vars;
pub mod cli;
pub mod vectors;
pub mod loops;
pub mod structs;
pub mod conditionals;
pub mod enums;
pub mod pointer_ref;

pub fn run() {
    println!("Days 1-3: Getting Started");
    print::run();
    strings::run();
    functions::run();
    types::run();
    arrays::run();
    vars::run();
    cli::run();
    vectors::run();
    loops::run();
    structs::run();
    conditionals::run();
    enums::run();
    pointer_ref::run();
}