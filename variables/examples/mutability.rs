fn main() {
    println!("🎯 Mutability: `let mut`");
    let mut num: i32 = 5;
    println!("💡 Num is: {num}"); // 5
    num = 6;
    println!("💡 Num is: {num}"); // 6

    // constant
    println!("🎯 Mutability: `const`");
    const A: char = 'A';
    println!("💡 The value of A is: {A}");
}
