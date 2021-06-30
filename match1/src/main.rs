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

    let c1 = Coin::Penny;

    println!("Value of c1: {}", value_in_cents(c1));
    
    let c2 = Coin::Quarter(UsState::Alaska);

    println!("Value of c2: {}", value_in_cents(c2));
}