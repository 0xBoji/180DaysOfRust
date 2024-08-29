use std::fs::File;
use std::io::Read;

pub fn examples() {
    println!("Error Handling Examples:");
    result_example();
    custom_error_example();
}

fn result_example() {
    let file_result = File::open("nonexistent.txt");
    match file_result {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            println!("File content: {}", content);
        }
        Err(error) => println!("Error opening file: {:?}", error),
    }
}

#[derive(Debug)]
struct CustomError(String);

fn custom_error_example() {
    fn may_fail(fail: bool) -> Result<(), CustomError> {
        if fail {
            Err(CustomError("Something went wrong".to_string()))
        } else {
            Ok(())
        }
    }

    println!("Success: {:?}", may_fail(false));
    println!("Failure: {:?}", may_fail(true));
}