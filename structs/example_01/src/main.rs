struct User {
    username: String,
    email: String,
    sign_in_account: u64,
    active: bool,
}

fn main() {

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let user1: User = User {
        email: String::from("yohan@mail.com"),
        username: String::from("yohan"),
        active: true,
        sign_in_account: 1
    };

    let name1: String =user1.username;
    user1.username = String::from("thom");

    let user2: User = build_user(
        email: String::from("kyle@mail.com"), 
        username: String::from("kyle123")
    );

    let user3: User = User {
        email: String::from("james@mail.com"),
        username: String::from("james123"),
        ..user2
    };

}

fn bulid_user(email: String, username: String) -> User {
    User { 
        email,
        username,
        active: true,
        sign_in_account: 1,
    }
}