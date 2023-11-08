fn main() {
    println!("🎯 Using `if let...` statement");

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum: i8;
    if let Some(y) = y {
        sum = x + y;
    } else {
        sum = x;
    }
    println!("💡 Sum: {sum}");
}
