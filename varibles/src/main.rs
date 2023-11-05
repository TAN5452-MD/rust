#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main(){
    value_in_cents(coin::Quarter(UsState::Alabama));
}

fn value_in_cents(coin: coin) -> u8 {
    match coin{
        coin::Penny => 1,
        coin::Nickel => 5,
        coin::Dime => 10,
        coin::Quarter(state) => {
            println!("{:?}",state);
            25
        },
    }
}


