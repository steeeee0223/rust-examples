fn main() {
    println!("🎯 Strings - Create");

    let s: String = String::new();
    println!("💡 String s: {s}");

    let s: String = String::from("this is a string");
    println!("💡 String s: {s}");

    let s: String = "this is also a string".to_string();
    println!("💡 String s: {s}");
}
