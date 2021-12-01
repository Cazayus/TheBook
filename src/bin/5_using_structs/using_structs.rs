pub(crate) struct User {
    pub(crate) _active: bool,
    pub(crate) _username: String,
    pub(crate) email: String,
    pub(crate) _sign_in_count: u64,
}
pub(crate) fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        _username: String::from("someusername123"),
        _active: true,
        _sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    let _user1 = build_user_2(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    //let user2 = User {
    //    active: user1.active,
    //    username: user1.username,
    //    email: String::from("another@example.com"),
    //    sign_in_count: user1.sign_in_count,
    //};
    let _user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    struct AlwaysEqual;
    let _subject = AlwaysEqual;
    println!("okokok")
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        _username: username,
        _active: true,
        _sign_in_count: 1,
    }
}

fn build_user_2(email: String, _username: String) -> User {
    User {
        email,
        _username,
        _active: true,
        _sign_in_count: 1,
    }
}
