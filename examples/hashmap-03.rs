use std::collections::HashMap;

fn main() {
    println!("🎯 Hash Map - Iterate");

    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("yellow"), 10);
    scores.insert(String::from("blue"), 50);

    for (key, val) in &scores {
        println!("💡 [{key}]: {val}");
    }
}
