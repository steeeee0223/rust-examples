use std::fs::File;
use std::io::ErrorKind;

fn main() {
    println!("🎯 Exceptions - Handle Different Types of Errors");

    let path = "data/hello.txt";

    let greeting_file_result: Result<File, std::io::Error> = File::open(path);

    let greeting_file: File = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(fc) => fc,
                Err(e) => panic!("🚨 Error occurred while creating file: {:?}", e),
            },
            other_error => {
                panic!("🚨 Error occurred while opening file: {:?}", other_error);
            }
        },
    };

    println!("💡 File: {:?}", greeting_file);
}
