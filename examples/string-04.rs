fn main() {
    println!("🎯 Strings - Iterate");

    println!("💡 - Read characters");
    let hello = String::from("hello");

    for ch in hello.chars() {
        println!("💡 {ch}");
    }

    println!("💡 - Read bytes");
    for b in hello.bytes() {
        println!("💡 {b}");
    }
}
