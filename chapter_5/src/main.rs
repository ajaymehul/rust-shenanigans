struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}


fn main() {
    let mut user: User = build_user(String::from("Bob@rust.com"), String::from("Bob"));
    user.username = String::from("Bobby");
    println!("{}", user.username);

    let mut user2 = User {
        email: String::from("bobby_new@rust.com"),
        ..user
    };
    println!("{}", user2.username);
    user2.username = String::from("Bobbdy_Upgraded");
    println!("user2.username: {}", user2.username);
    user.email = String::from("new@rust.com");
    println!("user.email: {}", user.email);
    println!("user2.email: {}", user2.email);
    // cannot use user.username because ownership was passed to User2q
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}