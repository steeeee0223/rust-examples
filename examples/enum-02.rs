#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    println!("🎯 Create a better enum type `IpAddr`");

    let localhost = IpAddr::V4(127, 0, 0, 1);
    println!("💡 {:?}", localhost);

    let loopback = IpAddr::V6(String::from("::1"));
    println!("💡 {:?}", loopback);
}
