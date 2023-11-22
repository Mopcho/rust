use std::fmt;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn main() {

    impl fmt::Display for User {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {} {} {})", self.active, self.username, self.email, self.sign_in_count)
        }
    }

    let user1 = User {
        active: true,
        username: String::from("Pesho"),
        email: String::from("pesho@abv.bg"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("mitko@abv.bg"),
        username: String::from("Mitko"),
        ..user1
    };

    println!("{user1}");

    println!("{user2}");
}
