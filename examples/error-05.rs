use std::fs::File;
use std::io::{Error, Read};

fn main() {
    println!("🎯 Exceptions - Error Propagation with `?` operator");

    let result: Result<String, Error> = read_username_from_file();

    println!("💡 Read username from file: {:?}", result);
}

fn read_username_from_file() -> Result<String, Error> {
    let path: &str = "data/hello-user.txt";
    let mut file: File = File::open(path)?;

    let mut username: String = String::new();

    file.read_to_string(&mut username)?;

    Ok(username)
}
