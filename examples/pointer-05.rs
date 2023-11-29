struct MySmartPointer {
    data: String,
}

impl Drop for MySmartPointer {
    fn drop(&mut self) {
        println!("💡 Dropped data: `{}`!", self.data);
    }
}

#[allow(unused_variables)]
fn main() {
    println!("🎯 Pointer 5 - Drop Manually");
    let c = MySmartPointer {
        data: String::from("some data"),
    };
    println!("💡 MySmartPointer created.");
    drop(c);
    println!("💡 MySmartPointer dropped before the end of main.");
}
