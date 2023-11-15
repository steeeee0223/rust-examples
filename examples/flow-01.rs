use rust_examples::math::is_even_number;

fn main() {
    println!("🎯 Using `if...else...` statements");

    let number: i32 = 3;

    if number < 5 {
        println!("💡 condition was true");
    } else {
        println!("💡 condition was false");
    }

    let result: bool = is_even_number(number);
    let msg: &str = if result { "even" } else { "odd" };
    println!("💡 The number {number} is {msg}!");
}
