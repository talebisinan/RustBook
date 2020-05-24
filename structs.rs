struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    is_active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 0,
        is_active: false,
    }
}

// struct Color(u32, u32, u32);
//
// struct Point(i32, i32, i32);

fn main() {
    let mut new_user = User {
        username: String::from("stalebi"),
        email: String::from("123123ads@gmail.com"),
        sign_in_count: 0,
        is_active: true,
    };
    println!("email: {}", new_user.email);
    new_user.email = String::from("asdcscxcz@protonmail.com");

    let new_user_02 = build_user(String::from("asdsa@asda.com"), String::from(
        "asdassd"));
    println!("email: {}", new_user.email);
    println!("email: {}", new_user_02.email);

    let new_user_03 = User {
        username: String::from("sth"),
        email: String::from("Sth@sth.com"),
        ..new_user
    };
    println!("email: {}", new_user_03.email);

    // let black = Color(0, 0, 0);
    // let origin = Point(0, 0, 0);
    //
    // println!("color: {:?}, point: {:?}", black, origin);
}
