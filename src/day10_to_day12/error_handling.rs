use std::fs::File;
use std::io::{self, Read};
use std::error::Error;
use std::fmt;

pub fn examples() -> Result<(), Box<dyn Error>> {
    println!("Error Handling Examples:");
    result_example()?;
    custom_error_example()?;
    Ok(())
}

fn result_example() -> Result<(), io::Error> {
    let file_result = File::open("nonexistent.txt");
    match file_result {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            println!("File content: {}", content);
            Ok(())
        }
        Err(error) => {
            println!("Error opening file: {:?}", error);
            Err(error)
        }
    }
}

#[derive(Debug)]
struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CustomError: {}", self.0)
    }
}

impl Error for CustomError {}

fn custom_error_example() -> Result<(), CustomError> {
    fn may_fail(fail: bool) -> Result<(), CustomError> {
        if fail {
            Err(CustomError("Something went wrong".to_string()))
        } else {
            Ok(())
        }
    }

    println!("Success: {:?}", may_fail(false)?);
    println!("Failure: {:?}", may_fail(true));
    Ok(())
}