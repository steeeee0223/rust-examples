use std::collections::HashMap;

fn main() {
    println!("🎯 Hash Map - Get (with default)");

    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("yellow"), 10);

    let color: String = String::from("yellow");
    let score: i32 = scores.get(&color).copied().unwrap_or(-1);
    println!("💡 Score of {color}: {:?}", score);

    let score: &i32 = scores.get("blue").unwrap_or(&-1);
    println!("💡 Score of blue: {:?}", score);
}
