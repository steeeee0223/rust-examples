fn main() {
    println!("🎯 Vectors - Iterate");

    println!("💡 - Read elements");
    let v: Vec<i32> = vec![100, 32, 57, 5, 22, 49];
    for i in &v {
        println!("💡 {i}");
    }

    println!("💡 - Map elements in place");
    let mut v2: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("💡 v2: {:?}", v2);
    for i in &mut v2 {
        *i *= 2;
    }
    println!("💡 double of v2: {:?}", v2);
}
