#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    println!("🎯 Create methods / associated functions in a struct");
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("💡 rect {:#?}", rect);
    dbg!(&rect);

    if rect.width() {
        println!("💡 rect has a positive width: {}", rect.width);
        println!("💡 Area of rect: {}", rect.area());
    }

    /* Can a rectangle hold another rectangle? */
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("💡 Can rect hold rect2? {}", rect.can_hold(&rect2));
    println!("💡 Can rect hold rect3? {}", rect.can_hold(&rect3));

    /* Construct a square */
    let square = Rectangle::square(5);
    println!("💡 Construct a square {:?}", square);
}
