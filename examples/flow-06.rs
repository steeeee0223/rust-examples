fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!("🎯 Match with `Option<T>`");

    let five: Option<i32> = Some(5);
    println!("💡 Some(5) => {:?}", five);
    let six: Option<i32> = plus_one(five);
    println!("💡 plus_one(five) => {:?}", six);
    let none: Option<i32> = plus_one(None);
    println!("💡 plus_one(None) => {:?}", none);
}
