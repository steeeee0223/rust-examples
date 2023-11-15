fn main() {
    println!("🎯 Strings - Read element");

    let hello = String::from("hello");
    println!("💡 String: {hello}");

    let s = &hello[0..1];
    println!("💡 Slice &hello[0..3]: {s}");

    let hello = "Здравствуйте";
    println!("💡 String: {hello}");
    println!("💡 hello.chars(): {:?}", hello.chars());
    println!("💡 hello.bytes(): {:?}", hello.bytes());
}
