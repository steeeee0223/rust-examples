fn main() {
    println!("🎯 Pointer 2 - Dereference");

    let x: i32 = 5;
    let y: Box<i32> = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
