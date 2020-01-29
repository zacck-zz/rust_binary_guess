// entry point into the program
fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    println!(
        "The home address is: {:?} and the loopback is: {:?}",
        home, loopback
    );
}

//define an IP enum
#[derive(Debug)]
enum IpAddrKind {
    // Add data into enum variants to be more concise
    // This way each variant can take in as many arguments as it needs
    // negating the need for another datastructure such as a record or map
    V4(u8, u8, u8, u8),
    V6(String),
}
