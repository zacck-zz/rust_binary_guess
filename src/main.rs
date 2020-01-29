// entry point into the program
fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    println!(
        "The home address is: {:?} and the loopback is: {:?}",
        home, loopback
    );

    let q = Message::Quit;
    let mov = Message::Move { x: 45, y: 67 };
    let write = Message::Write(String::from("My name is"));
    let color = Message::ChangeColor(34, 55, 77);

    println!(
        "The moves are move: {:?}, then write: {:?}, then change color: {:?}, then quit: {:?}",
        mov, write, color, q,
    );

    let _penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let _dime = Coin::Dime;
    let q1 = Coin::Quarter(UsState::Alabama);
    let q2 = Coin::Quarter(UsState::Alaska);

    println!(
        "A nickel has {} cents in it. \n Q1  has {} cents in it \n Q2 has {} cents in it",
        value_in_cents(nickel),
        value_in_cents(q1),
        value_in_cents(q2)
    );

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    match six {
        None => println!("None"),
        Some(six) => println!("{}", six),
    }

    match none {
        None => println!("True none has none"),
        Some(_) => println!("Wow impossible there is something in here!"),
    }
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

// Define a message enum
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State the quarter is from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
