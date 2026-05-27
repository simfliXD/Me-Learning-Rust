#![allow(warnings)]
fn main() {
    //let rectangle = (200, 250);

    // Struct
    struct Book {
        title: String,
        author: String,
        pager: u32,
        available: bool,
    }
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1: User = User {
        active: true,
        username: String::from("A_username"),
        email: String::from("example@example.org"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.org");
    println!("User email is {}", user1.email);

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }
    let user3 = build_user(
        String::from("another@mail.com"),
        String::from("another_username"),
    );

    let user2 = User {
        email: String::from("another@m.com"),
        ..user1
    };
    println!("User 2 email is {}", user2.email);
    println!("User 3 email is {}", user3.email);

    //Tuple Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    // Unit-like Struct
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}
