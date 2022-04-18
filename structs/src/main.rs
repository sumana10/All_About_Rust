

struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    
    let mut user1 = User{
        email: String::from("bogdan@gmail.com"),
        username: String::from("bogdan123"),
        active: true,
        sign_in_count: 1
    };

    let name: String = user1.username;

    user1.username = String::from("wallace123");

    let user2 = build_user(
        String::from("kylie@mail.com"), 
        String::from("kyle123")
    );

    let user3 = User{
        email: String::from("james@gmail.com"),
        username: String::from("james123"),
        ..user2
    };
}

fn build_user(email: String, username: String) -> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}