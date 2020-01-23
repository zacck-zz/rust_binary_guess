// entry point into the program
fn main() {
    //create an instance of a struct
    let mut user1 = User {
        email: String::from("me@me.com"),
        username: String::from("superbike_z"),
        active: true,
        sign_in_count: 1,
    };

    // access instance variables
    println!(
        "Uname, email, active, sign_count: {} {} {} {}",
        user1.username, user1.email, user1.active, user1.sign_in_count
    );

    // update mutable instance variables using dot notation
    user1.sign_in_count = 2;

    println!("Updated sign in count: {}", user1.sign_in_count);

    let mut user2 = build_user(String::from("her@her.com"), String::from("dino_c"));

    println!("User2 name is: {}", user2.username);

    user2.sign_in_count = 2;
}

// sample struct for a user type
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        // init shorthand when variables and fields have the same name
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
