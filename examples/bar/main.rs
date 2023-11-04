mod util;

fn main() {
    util::logger(String::from("[EXAMPLE] bar"));
    let x: i32 = 5;
    util::logger(format!("The value of x is: {0}", x)); // 5
}
