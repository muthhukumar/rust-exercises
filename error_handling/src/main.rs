use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    // let greeting_file =
    //     File::open("hello.txt").expect("hello.text file should be provided in the project");
    //
    let username = match read_username_from_file() {
        Ok(value) => value,
        Err(e) => panic!("{:?}", e),
    };

    println!("username is {}", username)
}

fn read_username_from_file_with_question_operator() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file = File::open("hello.txt");

    let mut file = match username_file {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => return Err(e),
    }
}

fn read_file_with_match() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(f) => f,
                Err(err) => panic!("Failed to create file. Reason: {}", err),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

fn this_will_fail() {
    let x = vec![1, 2, 3];
    x[20];
}
