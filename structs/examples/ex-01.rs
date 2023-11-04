use structs::build_user;
use structs::User;

fn main() {
    println!("🎯 This shows how we create and modify a struct");

    /* construct a `User` */
    let mut user1: User = build_user(String::from("user"), String::from("user@example.com"));
    let name1 = user1.username;
    println!("💡 User 1 - name: {name1}");

    /* modify the property of `User` */
    user1.username = String::from("steve");
    println!("💡 User 1 - name (modified): {}", user1.username);

    /* construct a new `User` from another `User` */
    let user2: User = User {
        username: String::from("john"),
        ..user1
    };
    println!("💡 User 2 - name: {}", user2.username);
    println!("💡 User 2 - email: {}", user2.email);
}
