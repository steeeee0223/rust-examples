use std::collections::HashMap;

fn main() {
    println!("🎯 Hash Map - Word count");

    let text = "hello world wonderful world";
    let mut counter: HashMap<char, i32> = HashMap::new();

    for ch in text.chars() {
        if ch == ' ' {
            continue;
        }
        *counter.entry(ch).or_insert(0) += 1;
    }
    println!("💡 counter: {:?}", counter);
}
