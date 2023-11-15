fn main() {
    println!("🎯 Vectors - Read element");
    let v = vec![1, 2, 3, 4, 5];
    println!("💡 Vector v: {:?}", v);

    let third: &i32 = &v[2];
    println!("💡 The third element is {third}");

    let index = 100;
    let element: Option<&i32> = v.get(index);
    match element {
        Some(element) => println!("💡 The {index}-th element is {element}"),
        None => println!("💡 There is no {index}-th element."),
    }
}
