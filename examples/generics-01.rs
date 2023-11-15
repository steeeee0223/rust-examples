#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    println!("🎯 Generics - Struct");

    let p: Point<i32> = Point { x: 5, y: 10 };

    println!("💡 p.x = {}", p.x());
}
