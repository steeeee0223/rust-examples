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
    println!("🎯 Pointer 4 - Implement Drop");

    let c = MySmartPointer {
        data: String::from("my stuff"),
    };
    let d = MySmartPointer {
        data: String::from("other stuff"),
    };
    println!("💡 MySmartPointer created.");
}
