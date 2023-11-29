enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

#[allow(unused_variables)]
fn main() {
    println!("🎯 Pointer 7 - Reference Counting");

    let a: Rc<List> = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("💡 Count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("💡 Count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("💡 Count after creating c = {}", Rc::strong_count(&a));
    }
    println!(
        "💡 Count after c goes out of scope = {}",
        Rc::strong_count(&a)
    );
}
