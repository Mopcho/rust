use std::{collections::HashMap, io::stdin, process::exit};

const YELLOW_OPENING: &str = "\x1b[93m";
const YELLOW_CLOSING: &str = "\x1b[0m";

enum Command {
    Add { dept: String, name: String },
    All,
    Exit,
}

impl Command {
    fn from_input(s: &str) -> Option<Self> {
        let words: Vec<&str> = s.split_whitespace().collect();
        match words.as_slice() {
            ["Add", name, "to", dept] => Some(Command::Add {
                name: name.to_string(),
                dept: dept.to_string(),
            }),
            ["All"] => Some(Command::All),
            ["Exit"] => Some(Command::Exit),
            _ => None,
        }
    }
}

fn main() {
    let mut hash_map: HashMap<String, Vec<String>> = HashMap::from([]);

    loop {
        println!("{YELLOW_OPENING}Enter an command{YELLOW_CLOSING}");

        let mut input = String::new();

        stdin().read_line(&mut input).expect("Something went wrong");

        match Command::from_input(input.trim()) {
            Some(Command::All) => println!("{:?}", hash_map),
            Some(Command::Add { dept, name }) => hash_map.entry(dept).or_default().push(name),
            Some(Command::Exit) => {
                println!("Exiting...");
                break;
            }
            None => println!("Input error"),
        }
    }
}
