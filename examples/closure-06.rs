#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    println!("🎯 Closure - 6. Sorting");

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r: &Rectangle| {
        num_sort_operations += 1;
        r.width
    });

    println!("💡 {:#?}, sorted in {num_sort_operations} operations", list);
}
