use std::cell::RefCell;

fn main() {
    println!("🎯 Pointer 8 - Interior Mutability");

    let x: RefCell<i32> = RefCell::new(5);
    println!("💡 x: {:?}", x);

    *x.borrow_mut() += 1;
    println!("💡 x: {:?}", x);
}
