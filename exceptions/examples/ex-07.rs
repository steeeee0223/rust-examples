fn main() {
    println!("🎯 Exceptions - Practice");

    let res: Result<u32, &str> = divide(3, 2);
    println!("💡 3 / 2 = {:?}", res.unwrap());
    let res: Result<u32, &str> = divide(3, 0);
    println!("💡 3 / 0 = {:?}", res);
}

fn divide(x: u32, y: u32) -> Result<u32, &'static str> {
    if y == 0 {
        Err("Zero divisor error")
    } else {
        Ok(x / y)
    }
}
