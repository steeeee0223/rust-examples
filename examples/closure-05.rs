use std::thread;

fn main() {
    println!("🎯 Closure - 5. Threading");

    let list = vec![1, 2, 3];
    println!("💡 Before defining closure: {:?}", list);

    thread::spawn(move || println!("💡 From thread: {:?}", list))
        .join()
        .unwrap();
}
