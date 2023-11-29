#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    println!("🎯 Pointer 6 - Reference Counting");

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("💡 List a = {:?}", a);

    let b: List = Cons(3, Rc::clone(&a));
    let c: List = Cons(4, Rc::clone(&a));

    println!("💡 List b = {:?}", b);
    println!("💡 List c = {:?}", c);
}
