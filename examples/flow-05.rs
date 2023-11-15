fn main() {
    println!("🎯 Using `for` loop");
    let a: [i32; 5] = [10, 20, 30, 40, 50];

    for element in a {
        println!("💡 The value is: {element}");
    }
    println!("💡 Loop complete!");

    for number in (1..4).rev() {
        println!("💡 The value is: {number}");
    }
    println!("💡 Loop complete!");
}
