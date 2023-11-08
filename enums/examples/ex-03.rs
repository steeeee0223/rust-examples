#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        match self {
            Message::Write(msg) => println!("💡 Writing message: {msg}"),
            Message::Quit => println!("💡 Quit!"),
            Message::Move { x, y } => println!("💡 Moving to: ({x}, {y})"),
            Message::ChangeColor(r, g, b) => {
                println!("💡 Changing color into: rgb({r}, {g}, {b})")
            }
        }
    }
}

fn main() {
    println!("🎯 Implement an enum type and match!");

    let mut m = Message::Write(String::from("hello"));
    m.call();
    m = Message::ChangeColor(255, 255, 255);
    m.call();
    m = Message::Move { x: 0, y: 0 };
    m.call();
    m = Message::Quit;
    m.call();
}
