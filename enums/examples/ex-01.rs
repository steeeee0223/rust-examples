#![allow(dead_code)]
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    println!("🎯 Create an enum type");

    let mut ip_type: IpAddrKind = IpAddrKind::V4;
    println!("💡 {:?}", ip_type);
    ip_type = IpAddrKind::V6;
    println!("💡 {:?}", ip_type);

    let localhost = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("💡 {:?}", localhost);

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("💡 {:?}", loopback);
}
