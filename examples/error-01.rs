use std::fs::File;
use std::io::Error;

fn main() {
    println!("🎯 Exceptions - Handle Error");

    let greeting_file_result: Result<File, Error> = File::open("hello.txt");

    let greeting_file: File = match greeting_file_result {
        Ok(file) => file,
        Err(e) => panic!("🚨 Error occurred while opening file: {:?}", e),
    };

    println!("💡 File: {:?}", greeting_file);
}
