use std::ops::Deref;

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    println!("🎯 Pointer 3 - Implement Deref");

    let x: i32 = 5;
    let y: MyBox<i32> = MyBox::new(x);

    println!("💡 y = {:?}", y);
    // it runs `*(y.deref())`
    println!("💡 *y = {}", *y);
}
