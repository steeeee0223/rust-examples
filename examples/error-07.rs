fn main() {
    println!("🎯 Exceptions - Practice");

    let res: Result<u32, String> = divide(3, 2);
    println!("💡 3 / 2 = {:?}", res.unwrap());
    let res: Result<u32, String> = divide(3, 0);
    println!("💡 3 / 0 = {:?}", res);
}

fn divide(x: u32, y: u32) -> Result<u32, String> {
    if y == 0 {
        Err(String::from("Zero divisor error"))
    } else {
        Ok(x / y)
    }
}
