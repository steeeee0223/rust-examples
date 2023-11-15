fn main() {
    println!("🎯 Return value from `loop`");
    let mut counter: i32 = 0;

    let result: i32 = loop {
        counter += 1;
        println!("💡 counter: {counter}");

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("💡 The result is {result}"); // 20
}
