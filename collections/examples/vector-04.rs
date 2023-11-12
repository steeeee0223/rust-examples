fn main() {
    println!("🎯 Vectors - Read element (Ownership)");
    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("💡 Vector v: {:?}", v);
    {
        let first: &i32 = &v[0];
        println!("💡 First element of v: {first}");
    }
    v.push(6);
    println!("💡 Vector v: {:?}", v);
}
