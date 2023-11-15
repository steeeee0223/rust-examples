fn main() {
    println!("🎯 Re-define: `let`");
    let x: i32 = 5;
    let x: i32 = x + 1;
    {
        let x: i32 = x * 2;
        println!("💡 The value of x in the inner scope is: {x}"); // 12
    }
    println!("💡 The value of x is: {x}"); // 6
}
