#[derive(Debug)]
enum UsState {
    Michigan,
    California
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from state: {:?}", state);
            25
        }
    }
}


fn main() {
    let coin_1 = Coin::Quarter(UsState::Michigan);
    println!("coin_1 value: {}", value_in_cents(&coin_1));
    println!("coin_1 value: {}", value_in_cents(&coin_1));

}
