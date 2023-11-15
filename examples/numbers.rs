fn main() {
    println!("🎯 This shows some operations in numbers");
    let x: u8 = b'A';
    println!("💡 The value of x is :{x}"); // 65

    // addition
    let sum: i32 = 5 + 10;
    println!("💡 The sum: {sum}");

    // subtraction
    let difference: f64 = 95.5 - 4.3;
    println!("💡 The difference: {difference}");

    // multiplication
    let product: i32 = 4 * 30;
    println!("💡 The product: {product}");

    // division
    let quotient: f64 = 56.7 / 32.2;
    println!("💡 The quotient: {quotient}");
    let truncated: i32 = -5 / 3; // Results in -1
    println!("💡 The truncated: {truncated}");

    // remainder
    let remainder: i32 = 43 % 5;
    println!("💡 The remainder: {remainder}");
}
