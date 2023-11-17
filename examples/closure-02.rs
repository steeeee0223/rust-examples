fn main() {
    println!("🎯 Closure - 2");

    let double = |x| x * 2;

    let mut array = [1, 2, 3, 4, 5];
    println!("💡 Original array: {:?}", array);
    array = array.map(double);
    println!("💡 Double array: {:?}", array);
}
