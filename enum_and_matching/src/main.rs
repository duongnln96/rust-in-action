#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// need to cover all case in enum with match
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {

    // Option example

    let x = Some(5);
    let y: Option<i32> = None;

    match x {
        Some(value) => println!("x has value: {}", value),
        None => println!("x has no value"),
    }

    match y {
        Some(value) => println!("y has value: {}", value),
        None => println!("y has no value"),
    }
}

