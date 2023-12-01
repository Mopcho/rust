use std::fs::File;
use std::io::{self, Read};
use std::panic;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    let answer = match read_username_from_file() {
        Ok(value) => value,
        Err(err) => panic!("{err}"),
    };

    println!("{answer}");
}
