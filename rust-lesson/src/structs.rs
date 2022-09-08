#[derive(Debug)]
struct User {
    username: String,
    email: String,
    login_count: i64,
    active: bool,
}
/**
 *
 * its a cool feature in rust.
 * */
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}
impl Rectangle {
    fn set(height: u32, width: u32) -> Self {
        Self { height, width }
    }
    fn area(&self) -> u32 {
        let ans = self.height * self.width;
        ans
    }
}
pub fn run() {
    let user1 = User {
        username: String::from("foo"),
        email: String::from("fooisbar@rs.com"),
        login_count: 11,
        active: true,
    };
    // let user2 = user1; // ownership move to user2
    // println!("{}\r\n", user1); // can not use user1
    let mut user1 = User {
        username: String::from("foo"),
        email: String::from("fooisbar@rs.com"),
        login_count: 11,
        active: true,
    };
    user1.username = String::from("bar");
    println!("{:#?}", user1);
    let user2 = set_user(String::from("foobar"), String::from("barisfoo@rs.com"));
    println!("{:#?}", user2);

    let rect = Rectangle::set(11, 11);
    println!("{:#?}", rect);
    println!("{}", rect.area());
}
fn set_user(username: String, email: String) -> User {
    User {
        username,
        email,
        login_count: 1,
        active: true,
    }
}
