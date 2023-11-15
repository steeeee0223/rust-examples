fn main() {
    println!("🎯 Vectors - Update");

    let mut v: Vec<i32> = Vec::new();
    println!("💡 Vector v: {:?}", v);

    v.push(1);
    v.push(3);
    v.push(5);
    v.push(2);
    println!("💡 Vector v: {:?}", v);

    let mut v2: Vec<i32> = vec![1, 2, 3];
    println!("💡 Vector v2: {:?}", v2);
    v2.push(1);
    println!("💡 Vector v2: {:?}", v2);
}
