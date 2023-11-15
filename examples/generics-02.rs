#[derive(Debug)]
struct Point<X, Y> {
    x: X,
    y: Y,
}

impl<X, Y> Point<X, Y> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    println!("🎯 Generics - Struct implementation");

    let p1: Point<i32, i32> = Point { x: 5, y: 10 };
    let p2: Point<&str, &str> = Point { x: "Hello", y: "c" };

    let p3: Point<i32, &str> = p1.mixup(p2);
    println!("💡 p3 = {:?}", p3);
}
