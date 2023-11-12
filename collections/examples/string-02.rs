fn main() {
    println!("🎯 Strings - Update");

    let mut s: String = String::from("hello");
    println!("💡 String s: {s}");
    let world: &str = " world";
    s.push_str(world);
    s.push('!');
    println!("💡 String s: {s}");

    let s2: String = String::from(" hello rust!");
    let s3: String = s + &s2; // note s1 has been moved here and can no longer be used
    println!("💡 String s3: {s3}");
    println!("💡 String s2: {s2}");
}
