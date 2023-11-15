use std::collections::HashMap;

fn main() {
    println!("🎯 Hash Map - Set");

    let mut scores: HashMap<String, i32> = HashMap::new();
    println!("💡 scores: {:?}", scores);

    scores.insert(String::from("yellow"), 10);
    scores.insert(String::from("blue"), 50);
    println!("💡 scores: {:?}", scores); // yello -> 10, blue -> 50

    scores.insert(String::from("blue"), 30);
    println!("💡 scores: {:?}", scores); // blue -> 30

    scores.entry(String::from("blue")).or_insert(40);
    println!("💡 scores: {:?}", scores); // blue -> 30

    let color = String::from("green");
    scores.insert(color, 40); // `color` is removed
    println!("💡 scores: {:?}", scores); // green -> 40
}
