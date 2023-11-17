fn main() {
    println!("🎯 Closure - 4");

    let mut list = vec![1, 2, 3];

    println!("💡 Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("💡 After calling closure: {:?}", list);
}
