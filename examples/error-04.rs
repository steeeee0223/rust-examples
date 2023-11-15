use std::fs::File;
use std::io::{Error, Read};

fn main() {
    println!("🎯 Exceptions - Error Propagation");

    let result: Result<String, Error> = read_username_from_file();

    println!("💡 Read username from file: {:?}", result);
}

fn read_username_from_file() -> Result<String, Error> {
    let path: &str = "data/hello-user.txt";
    let file_result: Result<File, Error> = File::open(path);

    let mut file: File = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username: String = String::new();

    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
