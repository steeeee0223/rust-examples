fn main() {
    println!("🎯 This shows some operations in tuple type");
    let tuple: (&str, i32) = ("Hello Rust!", 1_000);
    println!("💡 The 2nd index of tuple is: {}", tuple.1);
    let (message, ..) = tuple;
    println!("💡 The unpacked message is: {message}");
}
