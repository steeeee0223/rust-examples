use std::fs::File;
use std::io::Error;

fn main() {
    println!("🎯 Exceptions - Handle Results with unwrap/expect");

    let path = "data/hello-rust.txt";

    let greeting_file_result: Result<File, Error> = File::open(path);

    let error: Error = greeting_file_result.unwrap_err();

    println!("💡 Error kind: {:?}", error.kind());

    let new_file: File = File::create(path).unwrap();

    println!("💡 New file: {:?}", new_file);
}
